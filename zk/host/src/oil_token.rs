use axum::{ http::StatusCode, Json };
use oil_tokenization_core::OilTokenState;
use chrono::Utc;
use entity::{ oil_token, OilTokenModel };
use ethers::types::Address;
use methods::{ INIT_OIL_TOKEN_ELF, INIT_OIL_TOKEN_ID };
use risc0_zkvm::{ default_prover, ExecutorEnv, Receipt };
use sea_orm::{ ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, QueryOrder };
use serde::{ Deserialize, Serialize };
use serde_json::{ Value, json };
use std::sync::Arc;

use crate::{ auth::USER, jwt::Claims, SessionStats };

pub fn get_oil_token_leaves(oil_tokens: &Vec<OilTokenModel>) -> Vec<String> {
    let mut leaves = vec![];
    for oil_token in oil_tokens {
        let oil_token_record = format!(
            "{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            oil_token.id,
            oil_token.token_id,
            oil_token.oil_type,
            oil_token.grade,
            oil_token.quantity,
            oil_token.unit,
            oil_token.location,
            oil_token.certification,
            oil_token.quality_report.to_string(),
            oil_token.storage_conditions.to_string(),
            oil_token.expiry_date.and_utc().timestamp(),
            oil_token.current_price,
            oil_token.reserve_price,
            oil_token.status
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_default(),
            oil_token.owner,
            oil_token.created_at.and_utc().timestamp(),
            oil_token.updated_at.and_utc().timestamp()
        );
        leaves.push(oil_token_record);
    }
    leaves
}

#[derive(Serialize, Deserialize)]
pub struct OilTokenCommit {
    pub receipt: Receipt,
    pub stats: SessionStats,
}

impl OilTokenCommit {
    pub fn get_commit(&self) -> Result<OilTokenState, String> {
        let state = self.receipt.journal.decode().map_err(|e| e.to_string())?;
        Ok(state)
    }

    pub fn verify_and_get_commit(&self) -> Result<OilTokenState, String> {
        self.receipt.verify(INIT_OIL_TOKEN_ID).map_err(|e| e.to_string())?;
        self.get_commit()
    }
}

pub fn init_oil_token(leaves: Vec<String>) -> Result<OilTokenCommit, String> {
    let env = ExecutorEnv::builder().write(&leaves).unwrap().build().unwrap();
    let prover = default_prover();
    let prove_info = prover.prove(env, INIT_OIL_TOKEN_ELF).unwrap();
    let oil_token_commit = OilTokenCommit {
        receipt: prove_info.receipt,
        stats: SessionStats {
            segments: prove_info.stats.segments,
            total_cycles: prove_info.stats.total_cycles,
            user_cycles: prove_info.stats.user_cycles,
            paging_cycles: prove_info.stats.paging_cycles,
            reserved_cycles: prove_info.stats.reserved_cycles,
        },
    };
    Ok(oil_token_commit)
}

pub async fn init_oil_token_handler(axum::extract::State(
    db,
): axum::extract::State<Arc<sea_orm::DatabaseConnection>>) -> Result<
    Json<Value>,
    (axum::http::StatusCode, String)
> {
    let oil_tokens = get_all_oil_tokens(&db).await.unwrap();
    let leaves = get_oil_token_leaves(&oil_tokens);
    let result = init_oil_token(leaves).map_err(|e| (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        e.to_string(),
    ))?;
    Ok(Json(json!(result)))
}

#[axum::debug_handler]
pub async fn create_oil_token(
    axum::extract::State(db): axum::extract::State<Arc<sea_orm::DatabaseConnection>>,
    Json(oil_token_data): Json<OilTokenModel>
) -> Result<Json<Value>, (axum::http::StatusCode, String)> {
    let current_user = USER.with(|user| Claims {
        username: user.username.clone(),
        addr: user.addr
            .clone()
            .parse::<Address>()
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid address".to_string()))
            .unwrap()
            .to_string(),
        exp: user.exp,
    });

    eprintln!("Request from user: {:?}", current_user.addr);
    eprintln!("Request from username: {}", current_user.username);

    use sea_orm::ActiveValue::Set;
    let oil_token_id = oil_token::Entity
        ::find()
        .order_by_desc(oil_token::Column::Id)
        .one(&*db).await
        .unwrap()
        .unwrap();
    let now_naive: chrono::NaiveDateTime = Utc::now().naive_utc();
    let oil_token_model = oil_token::ActiveModel {
        id: Set(oil_token_id.id + 1),
        token_id: Set(oil_token_data.token_id),
        oil_type: Set(oil_token_data.oil_type.to_owned()),
        grade: Set(oil_token_data.grade.to_owned()),
        quantity: Set(oil_token_data.quantity),
        unit: Set(oil_token_data.unit.to_owned()),
        location: Set(oil_token_data.location.to_owned()),
        certification: Set(oil_token_data.certification.to_owned()),
        quality_report: Set(oil_token_data.quality_report.clone()),
        storage_conditions: Set(oil_token_data.storage_conditions.clone()),
        expiry_date: Set(oil_token_data.expiry_date),
        current_price: Set(oil_token_data.current_price),
        reserve_price: Set(oil_token_data.reserve_price),
        status: Set(oil_token_data.status.clone()),
        owner: Set(current_user.addr),
        created_at: Set(now_naive.clone()),
        updated_at: Set(now_naive),
        ..Default::default()
    };

    oil_token_model
        .insert(&*db).await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({
      "status": "success",
      "message": "Oil token created successfully"
  })))
}

// Handler to get an oil token by ID
pub async fn get_oil_token_by_id(
    axum::extract::Path(id): axum::extract::Path<i32>,
    axum::extract::State(db): axum::extract::State<Arc<sea_orm::DatabaseConnection>>
) -> Result<Json<Value>, (axum::http::StatusCode, String)> {
    let oil_token = oil_token::Entity
        ::find_by_id(id)
        .one(&*db).await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match oil_token {
        Some(oil_token) =>
            Ok(Json(json!({
        "status": "success",
        "data": oil_token
    }))),
        None => Err((axum::http::StatusCode::NOT_FOUND, "Oil token not found".to_string())),
    }
}

// Handler to get all oil tokens
pub async fn get_all_oil_tokens_handler(axum::extract::State(
    db,
): axum::extract::State<Arc<sea_orm::DatabaseConnection>>) -> Result<
    Json<Value>,
    (axum::http::StatusCode, String)
> {
    let oil_tokens = get_all_oil_tokens(&db).await.map_err(|e| (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        e.to_string(),
    ))?;

    Ok(Json(json!({
        "status": "success",
        "data": oil_tokens
    })))
}

// Function to get all oil tokens from database
pub async fn get_all_oil_tokens(
    db: &DatabaseConnection
) -> Result<Vec<::entity::oil_token::Model>, DbErr> {
    oil_token::Entity::find().order_by_asc(oil_token::Column::Id).all(db).await
}

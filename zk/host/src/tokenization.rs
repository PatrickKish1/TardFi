use axum::{ http::StatusCode, Json };
use oil_tokenization_core::TokenizationState;
use chrono::Utc;
use entity::{ tokenization, TokenizationModel };
use ethers::types::Address;
use methods::{ INIT_TOKENIZATION_ELF, INIT_TOKENIZATION_ID };
use risc0_zkvm::{ default_prover, ExecutorEnv, Receipt };
use sea_orm::{ ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, QueryOrder };
use serde::{ Deserialize, Serialize };
use serde_json::{ Value, json };
use std::sync::Arc;

use crate::{ auth::USER, jwt::Claims, SessionStats };

pub fn get_tokenization_leaves(tokenizations: &Vec<TokenizationModel>) -> Vec<String> {
    let mut leaves = vec![];
    for tokenization in tokenizations {
        let tokenization_record = format!(
            "{}:{}:{}:{}:{}:{}",
            tokenization.id,
            tokenization.oil_token_id,
            tokenization.tokenizer_id,
            tokenization.amount,
            tokenization.created_at.and_utc().timestamp(),
            tokenization.updated_at.and_utc().timestamp()
        );
        leaves.push(tokenization_record);
    }
    leaves
}

#[derive(Serialize, Deserialize)]
pub struct TokenizationCommit {
    pub receipt: Receipt,
    pub stats: SessionStats,
}

impl TokenizationCommit {
    pub fn get_commit(&self) -> Result<TokenizationState, String> {
        let state = self.receipt.journal.decode().map_err(|e| e.to_string())?;
        Ok(state)
    }

    pub fn verify_and_get_commit(&self) -> Result<TokenizationState, String> {
        self.receipt.verify(INIT_TOKENIZATION_ID).map_err(|e| e.to_string())?;
        self.get_commit()
    }
}

pub fn init_tokenization(leaves: Vec<String>) -> Result<TokenizationCommit, String> {
    let env = ExecutorEnv::builder().write(&leaves).unwrap().build().unwrap();
    let prover = default_prover();
    let prove_info = prover.prove(env, INIT_TOKENIZATION_ELF).unwrap();
    let tokenization_commit = TokenizationCommit {
        receipt: prove_info.receipt,
        stats: SessionStats {
            segments: prove_info.stats.segments,
            total_cycles: prove_info.stats.total_cycles,
            user_cycles: prove_info.stats.user_cycles,
            paging_cycles: prove_info.stats.paging_cycles,
            reserved_cycles: prove_info.stats.reserved_cycles,
        },
    };
    Ok(tokenization_commit)
}

pub async fn init_tokenization_handler(axum::extract::State(
    db,
): axum::extract::State<Arc<sea_orm::DatabaseConnection>>) -> Result<
    Json<Value>,
    (axum::http::StatusCode, String)
> {
    let tokenizations = get_all_tokenizations(&db).await.unwrap();
    let leaves = get_tokenization_leaves(&tokenizations);
    let result = init_tokenization(leaves).map_err(|e| (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        e.to_string(),
    ))?;
    Ok(Json(json!(result)))
}

#[axum::debug_handler]
pub async fn create_tokenization(
    axum::extract::State(db): axum::extract::State<Arc<sea_orm::DatabaseConnection>>,
    Json(tokenization_data): Json<TokenizationModel>
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
    let tokenization_id = tokenization::Entity
        ::find()
        .order_by_desc(tokenization::Column::Id)
        .one(&*db).await
        .unwrap()
        .unwrap();
    let now_naive: chrono::NaiveDateTime = Utc::now().naive_utc();
    let tokenization_model = tokenization::ActiveModel {
        id: Set(tokenization_id.id + 1),
        oil_token_id: Set(tokenization_data.oil_token_id),
        tokenizer_id: Set(current_user.addr),
        amount: Set(tokenization_data.amount),
        created_at: Set(now_naive.clone()),
        updated_at: Set(now_naive),
        ..Default::default()
    };

    tokenization_model
        .insert(&*db).await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(
        Json(
            json!({
      "status": "success",
      "message": "Tokenization created successfully"
  })
        )
    )
}

// Handler to get a tokenization by ID
pub async fn get_tokenization_by_id(
    axum::extract::Path(id): axum::extract::Path<i32>,
    axum::extract::State(db): axum::extract::State<Arc<sea_orm::DatabaseConnection>>
) -> Result<Json<Value>, (axum::http::StatusCode, String)> {
    let tokenization = tokenization::Entity
        ::find_by_id(id)
        .one(&*db).await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match tokenization {
        Some(tokenization) =>
            Ok(Json(json!({
        "status": "success",
        "data": tokenization
    }))),
        None => Err((axum::http::StatusCode::NOT_FOUND, "Tokenization not found".to_string())),
    }
}

// Handler to get all tokenizations
pub async fn get_tokenizations(axum::extract::State(
    db,
): axum::extract::State<Arc<sea_orm::DatabaseConnection>>) -> Result<
    Json<Value>,
    (axum::http::StatusCode, String)
> {
    let tokenizations = get_all_tokenizations(&db).await.map_err(|e| (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        e.to_string(),
    ))?;

    Ok(Json(json!({
        "status": "success",
        "data": tokenizations
    })))
}

// Function to get all tokenizations from database
pub async fn get_all_tokenizations(
    db: &DatabaseConnection
) -> Result<Vec<::entity::tokenization::Model>, DbErr> {
    tokenization::Entity::find().order_by_asc(tokenization::Column::Id).all(db).await
}

// Handler to complete a tokenization by ID
pub async fn complete_tokenization_by_id(
    axum::extract::Path(id): axum::extract::Path<i32>,
    axum::extract::State(db): axum::extract::State<Arc<sea_orm::DatabaseConnection>>
) -> Result<Json<Value>, (axum::http::StatusCode, String)> {
    let tokenization = tokenization::Entity
        ::find_by_id(id)
        .one(&*db).await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match tokenization {
        Some(tokenization) => {
            // Here you would implement the logic to complete the tokenization
            // For example, updating the oil token status, transferring ownership, etc.
            Ok(
                Json(
                    json!({
                "status": "success",
                "message": "Tokenization completed successfully",
                "data": tokenization
            })
                )
            )
        }
        None => Err((axum::http::StatusCode::NOT_FOUND, "Tokenization not found".to_string())),
    }
}

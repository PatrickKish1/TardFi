use axum::{ http::StatusCode, Json };
use oil_tokenization_core::CommentState;
use chrono::Utc;
use entity::{ comment, CommentModel };
use ethers::types::Address;
use methods::{ INIT_COMMENT_ELF, INIT_COMMENT_ID };
use risc0_zkvm::{ default_prover, ExecutorEnv, Receipt };
use sea_orm::{ ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, QueryOrder };
use serde::{ Deserialize, Serialize };
use serde_json::{ Value, json };
use std::sync::Arc;

use crate::{ auth::USER, jwt::Claims, SessionStats };

pub fn get_comment_leaves(comments: &Vec<CommentModel>) -> Vec<String> {
    let mut leaves = vec![];
    for comment in comments {
        let comment_record = format!(
            "{}:{}:{}:{}:{}:{}",
            comment.id,
            comment.oil_token_id,
            comment.user,
            comment.content,
            comment.created_at.and_utc().timestamp(),
            comment.updated_at.and_utc().timestamp()
        );
        leaves.push(comment_record);
    }
    leaves
}

#[derive(Serialize, Deserialize)]
pub struct CommentCommit {
    pub receipt: Receipt,
    pub stats: SessionStats,
}

impl CommentCommit {
    pub fn get_commit(&self) -> Result<CommentState, String> {
        let state = self.receipt.journal.decode().map_err(|e| e.to_string())?;
        Ok(state)
    }

    pub fn verify_and_get_commit(&self) -> Result<CommentState, String> {
        self.receipt.verify(INIT_COMMENT_ID).map_err(|e| e.to_string())?;
        self.get_commit()
    }
}

pub fn init_comment(leaves: Vec<String>) -> Result<CommentCommit, String> {
    let env = ExecutorEnv::builder().write(&leaves).unwrap().build().unwrap();
    let prover = default_prover();
    let prove_info = prover.prove(env, INIT_COMMENT_ELF).unwrap();
    let comment_commit = CommentCommit {
        receipt: prove_info.receipt,
        stats: SessionStats {
            segments: prove_info.stats.segments,
            total_cycles: prove_info.stats.total_cycles,
            user_cycles: prove_info.stats.user_cycles,
            paging_cycles: prove_info.stats.paging_cycles,
            reserved_cycles: prove_info.stats.reserved_cycles,
        },
    };
    Ok(comment_commit)
}

pub async fn init_comment_handler(axum::extract::State(
    db,
): axum::extract::State<Arc<sea_orm::DatabaseConnection>>) -> Result<
    Json<Value>,
    (axum::http::StatusCode, String)
> {
    let comments = get_all_comments(&db).await.unwrap();
    let leaves = get_comment_leaves(&comments);
    let result = init_comment(leaves).map_err(|e| (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        e.to_string(),
    ))?;
    Ok(Json(json!(result)))
}

#[axum::debug_handler]
pub async fn create_comment(
    axum::extract::State(db): axum::extract::State<Arc<sea_orm::DatabaseConnection>>,
    Json(comment_data): Json<CommentModel>
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
    let comment_id = comment::Entity
        ::find()
        .order_by_desc(comment::Column::Id)
        .one(&*db).await
        .unwrap()
        .unwrap();
    let now_naive: chrono::NaiveDateTime = Utc::now().naive_utc();
    let comment_model = comment::ActiveModel {
        id: Set(comment_id.id + 1),
        oil_token_id: Set(comment_data.oil_token_id),
        user: Set(current_user.addr),
        content: Set(comment_data.content.to_owned()),
        created_at: Set(now_naive.clone()),
        updated_at: Set(now_naive),
        ..Default::default()
    };

    comment_model
        .insert(&*db).await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({
      "status": "success",
      "message": "Comment created successfully"
  })))
}

// Handler to get comments by oil token ID
pub async fn get_comment_by_oil_token_id(
    axum::extract::Path(id): axum::extract::Path<i32>,
    axum::extract::State(db): axum::extract::State<Arc<sea_orm::DatabaseConnection>>
) -> Result<Json<Value>, (axum::http::StatusCode, String)> {
    let comments = comment::Entity
        ::find()
        .filter(comment::Column::OilTokenId.eq(id))
        .order_by_desc(comment::Column::CreatedAt)
        .all(&*db).await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({
        "status": "success",
        "data": comments
    })))
}

// Handler to get all comments
pub async fn get_comments(axum::extract::State(
    db,
): axum::extract::State<Arc<sea_orm::DatabaseConnection>>) -> Result<
    Json<Value>,
    (axum::http::StatusCode, String)
> {
    let comments = get_all_comments(&db).await.map_err(|e| (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        e.to_string(),
    ))?;

    Ok(Json(json!({
        "status": "success",
        "data": comments
    })))
}

// Function to get all comments from database
pub async fn get_all_comments(
    db: &DatabaseConnection
) -> Result<Vec<::entity::comment::Model>, DbErr> {
    comment::Entity::find().order_by_asc(comment::Column::Id).all(db).await
}

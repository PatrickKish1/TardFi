use axum::{ http::StatusCode, Json };
use oil_tokenization_core::{ OverallState, OverallParams };
use chrono::Utc;
use entity::{ oil_token, tokenization, comment };
use methods::{ INIT_OVERALL_ELF, INIT_OVERALL_ID };
use risc0_zkvm::{ default_prover, ExecutorEnv, Receipt };
use sea_orm::{ DatabaseConnection, DbErr, EntityTrait, QueryOrder };
use serde::{ Deserialize, Serialize };
use serde_json::{ Value, json };
use std::sync::Arc;

use crate::{ SessionStats, sync_state };

pub fn get_oil_token_leaves(oil_tokens: &Vec<::entity::oil_token::Model>) -> Vec<String> {
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

pub fn get_tokenization_leaves(tokenizations: &Vec<::entity::tokenization::Model>) -> Vec<String> {
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

pub fn get_comment_leaves(comments: &Vec<::entity::comment::Model>) -> Vec<String> {
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
pub struct OverallCommit {
    pub receipt: Receipt,
    pub stats: SessionStats,
}

impl OverallCommit {
    pub fn get_commit(&self) -> Result<OverallState, String> {
        let state = self.receipt.journal.decode().map_err(|e| e.to_string())?;
        Ok(state)
    }

    pub fn verify_and_get_commit(&self) -> Result<OverallState, String> {
        self.receipt.verify(INIT_OVERALL_ID).map_err(|e| e.to_string())?;
        self.get_commit()
    }
}

pub fn init_overall(
    oil_token_leaves: Vec<String>,
    tokenization_leaves: Vec<String>,
    comment_leaves: Vec<String>
) -> Result<OverallCommit, String> {
    let params = OverallParams {
        oil_token_leaves,
        tokenization_leaves,
        comment_leaves,
    };
    let env = ExecutorEnv::builder().write(&params).unwrap().build().unwrap();
    let prover = default_prover();
    let prove_info = prover.prove(env, INIT_OVERALL_ELF).unwrap();
    let overall_commit = OverallCommit {
        receipt: prove_info.receipt,
        stats: SessionStats {
            segments: prove_info.stats.segments,
            total_cycles: prove_info.stats.total_cycles,
            user_cycles: prove_info.stats.user_cycles,
            paging_cycles: prove_info.stats.paging_cycles,
            reserved_cycles: prove_info.stats.reserved_cycles,
        },
    };
    Ok(overall_commit)
}

pub async fn init_overall_handler(axum::extract::State(
    db,
): axum::extract::State<Arc<sea_orm::DatabaseConnection>>) -> Result<
    Json<Value>,
    (axum::http::StatusCode, String)
> {
    let oil_tokens = get_all_oil_tokens(&db).await.unwrap();
    let tokenizations = get_all_tokenizations(&db).await.unwrap();
    let comments = get_all_comments(&db).await.unwrap();

    let oil_token_leaves = get_oil_token_leaves(&oil_tokens);
    let tokenization_leaves = get_tokenization_leaves(&tokenizations);
    let comment_leaves = get_comment_leaves(&comments);

    let result = init_overall(oil_token_leaves, tokenization_leaves, comment_leaves).map_err(|e| (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        e.to_string(),
    ))?;
    Ok(Json(json!(result)))
}

pub async fn get_overall_state_handler(axum::extract::State(
    db,
): axum::extract::State<Arc<sea_orm::DatabaseConnection>>) -> Result<
    Json<Value>,
    (axum::http::StatusCode, String)
> {
    let oil_tokens = get_all_oil_tokens(&db).await.unwrap();
    let tokenizations = get_all_tokenizations(&db).await.unwrap();
    let comments = get_all_comments(&db).await.unwrap();

    let oil_token_leaves = get_oil_token_leaves(&oil_tokens);
    let tokenization_leaves = get_tokenization_leaves(&tokenizations);
    let comment_leaves = get_comment_leaves(&comments);

    let overall_state = OverallState::init(oil_token_leaves, tokenization_leaves, comment_leaves);

    Ok(Json(json!({
        "status": "success",
        "data": overall_state
    })))
}

pub async fn sync_state_handler(axum::extract::State(
    db,
): axum::extract::State<Arc<sea_orm::DatabaseConnection>>) -> Result<
    Json<Value>,
    (axum::http::StatusCode, String)
> {
    let oil_tokens = get_all_oil_tokens(&db).await.unwrap();
    let tokenizations = get_all_tokenizations(&db).await.unwrap();
    let comments = get_all_comments(&db).await.unwrap();

    let oil_token_leaves = get_oil_token_leaves(&oil_tokens);
    let tokenization_leaves = get_tokenization_leaves(&tokenizations);
    let comment_leaves = get_comment_leaves(&comments);

    let overall_commit = init_overall(
        oil_token_leaves,
        tokenization_leaves,
        comment_leaves
    ).map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let hash = sync_state(&overall_commit).await.map_err(|e| (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        e.to_string(),
    ))?;

    Ok(
        Json(
            json!({
        "status": "success",
        "hash": hash,
        "receipt": overall_commit.receipt,
        "stats": overall_commit.stats
    })
        )
    )
}

// Function to get all oil tokens from database
pub async fn get_all_oil_tokens(
    db: &DatabaseConnection
) -> Result<Vec<::entity::oil_token::Model>, DbErr> {
    oil_token::Entity::find().order_by_asc(oil_token::Column::Id).all(db).await
}

// Function to get all tokenizations from database
pub async fn get_all_tokenizations(
    db: &DatabaseConnection
) -> Result<Vec<::entity::tokenization::Model>, DbErr> {
    tokenization::Entity::find().order_by_asc(tokenization::Column::Id).all(db).await
}

// Function to get all comments from database
pub async fn get_all_comments(
    db: &DatabaseConnection
) -> Result<Vec<::entity::comment::Model>, DbErr> {
    comment::Entity::find().order_by_asc(comment::Column::Id).all(db).await
}

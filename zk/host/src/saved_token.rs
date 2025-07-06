use axum::{ http::StatusCode, Json };
use chrono::Utc;
use entity::{ saved_token, SavedTokenModel };
use ethers::types::Address;
use sea_orm::{ ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, QueryOrder };
use serde_json::{ Value, json };
use std::sync::Arc;

use crate::{ auth::USER, jwt::Claims };

#[axum::debug_handler]
pub async fn create_saved_token(
    axum::extract::State(db): axum::extract::State<Arc<sea_orm::DatabaseConnection>>,
    Json(saved_token_data): Json<SavedTokenModel>
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
    let saved_token_id = saved_token::Entity
        ::find()
        .order_by_desc(saved_token::Column::Id)
        .one(&*db).await
        .unwrap()
        .unwrap();
    let now_naive: chrono::NaiveDateTime = Utc::now().naive_utc();
    let saved_token_model = saved_token::ActiveModel {
        id: Set(saved_token_id.id + 1),
        user: Set(current_user.addr),
        oil_token_id: Set(saved_token_data.oil_token_id),
        created_at: Set(now_naive),
        ..Default::default()
    };

    saved_token_model
        .insert(&*db).await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({
      "status": "success",
      "message": "Token saved successfully"
  })))
}

// Handler to get saved tokens by user
pub async fn get_saved_tokens_by_user(
    axum::extract::Path(user): axum::extract::Path<String>,
    axum::extract::State(db): axum::extract::State<Arc<sea_orm::DatabaseConnection>>
) -> Result<Json<Value>, (axum::http::StatusCode, String)> {
    let saved_tokens = saved_token::Entity
        ::find()
        .filter(saved_token::Column::User.eq(user))
        .order_by_desc(saved_token::Column::CreatedAt)
        .all(&*db).await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({
        "status": "success",
        "data": saved_tokens
    })))
}

// Handler to get all saved tokens by oil token ID
pub async fn get_saved_tokens(
    axum::extract::Path(id): axum::extract::Path<i32>,
    axum::extract::State(db): axum::extract::State<Arc<sea_orm::DatabaseConnection>>
) -> Result<Json<Value>, (axum::http::StatusCode, String)> {
    let saved_tokens = saved_token::Entity
        ::find()
        .filter(saved_token::Column::OilTokenId.eq(id))
        .order_by_desc(saved_token::Column::CreatedAt)
        .all(&*db).await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({
        "status": "success",
        "data": saved_tokens
    })))
}

// Function to get all saved tokens from database
pub async fn get_all_saved_tokens(
    db: &DatabaseConnection
) -> Result<Vec<::entity::saved_token::Model>, DbErr> {
    saved_token::Entity::find().order_by_asc(saved_token::Column::Id).all(db).await
}

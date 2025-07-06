use std::sync::Arc;
use sea_orm::DatabaseConnection;
use serde::{ Deserialize, Serialize };

pub mod auth;
pub mod jwt;
pub mod oil_token;
pub mod tokenization;
pub mod comment;
pub mod saved_token;
pub mod overall;
pub mod redis;
pub mod sync_state;

#[derive(Serialize, Deserialize)]
pub struct SessionStats {
    pub segments: u32,
    pub total_cycles: u64,
    pub user_cycles: u64,
    pub paging_cycles: u64,
    pub reserved_cycles: u64,
}

pub type AppState = Arc<DatabaseConnection>;

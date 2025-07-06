use pinata_sdk::PinataApi;
use dotenv::dotenv;
use std::env;
use serde::{ Deserialize, Serialize };
use crate::overall::OverallCommit;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct SessionStats {
    /// Count of segments in this proof request
    pub segments: usize,

    /// Total cycles run within guest
    pub total_cycles: u64,

    /// User cycles run within guest
    pub user_cycles: u64,

    /// Paging cycles run within guest
    pub paging_cycles: u64,

    /// Reserved cycles run within guest
    pub reserved_cycles: u64,
}

use pinata_sdk::{ ApiError, PinByJson };

pub async fn sync_state(overall: &OverallCommit) -> Result<String, ApiError> {
    dotenv().ok();
    let api_key = env::var("PINATA_API_KEY").unwrap();
    let secret_key = env::var("PINATA_SECRET_KEY").unwrap();

    let api: PinataApi = PinataApi::new(api_key, secret_key).unwrap();

    let stats = SessionStats {
        segments: overall.stats.segments,
        total_cycles: overall.stats.total_cycles,
        user_cycles: overall.stats.user_cycles,
        paging_cycles: overall.stats.paging_cycles,
        reserved_cycles: overall.stats.reserved_cycles,
    };
    let new_overall = OverallCommit {
        receipt: overall.receipt.clone(),
        stats,
    };

    let result = api.pin_json(PinByJson::new(new_overall)).await;

    if let Ok(pinned_object) = result {
        let hash: String = pinned_object.ipfs_hash;
        return Ok(hash);
    } else {
        return Err(ApiError::GenericError("Failed while interacting with the api".to_string()));
    }
}

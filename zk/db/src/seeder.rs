use ::entity::{ oil_token, tokenization, comment, saved_token, TokenStatus };
use sea_orm::*;
// use serde_json::json;
use chrono::{ DateTime, Utc };
use sea_orm::ActiveValue::Set;
use serde::Deserialize;
use serde_json::Value;
use std::fs;
use serde_json::from_str;
use rust_decimal::{ prelude::FromPrimitive, Decimal };

#[derive(Deserialize)]
pub struct OilTokenSeed {
    pub id: i32,
    pub token_id: i32,
    pub oil_type: String,
    pub grade: String,
    pub quantity: f64,
    pub unit: String,
    pub location: String,
    pub certification: String,
    pub quality_report: Value,
    pub storage_conditions: Value,
    pub expiry_date: String,
    pub current_price: f64,
    pub reserve_price: f64,
    pub status: String,
    pub owner: String,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn seed_database(db: &DatabaseConnection) -> Result<(), DbErr> {
    println!("🌱 Starting oil tokenization database seeding...");

    // Clear existing data
    clear_database(db).await?;

    // Seed oil tokens
    seed_oil_tokens_from_json(db, "db/data/oil_tokens.json").await?;
    println!("✅ Seeded oil tokens");

    // Seed tokenizations
    seed_tokenizations(db).await?;
    println!("✅ Seeded tokenizations");

    // Seed comments
    seed_comments(db).await?;
    println!("✅ Seeded comments");

    // Seed saved tokens
    seed_saved_tokens(db).await?;
    println!("✅ Seeded saved tokens");

    println!("🎉 Oil tokenization database seeding completed successfully!");
    Ok(())
}

async fn clear_database(db: &DatabaseConnection) -> Result<(), DbErr> {
    // Clear in reverse order of dependencies
    saved_token::Entity::delete_many().exec(db).await?;
    comment::Entity::delete_many().exec(db).await?;
    tokenization::Entity::delete_many().exec(db).await?;
    oil_token::Entity::delete_many().exec(db).await?;
    Ok(())
}

pub async fn seed_oil_tokens_from_json(db: &DatabaseConnection, path: &str) -> Result<(), DbErr> {
    let data = fs::read_to_string(path).expect("Unable to read file");
    let oil_tokens: Vec<OilTokenSeed> = from_str(&data).expect("JSON was not well-formatted");

    for oil_token in oil_tokens {
        let model = oil_token::ActiveModel {
            id: Set(oil_token.id),
            token_id: Set(oil_token.token_id),
            oil_type: Set(oil_token.oil_type),
            grade: Set(oil_token.grade),
            quantity: Set(Decimal::from_f64(oil_token.quantity).unwrap()),
            unit: Set(oil_token.unit),
            location: Set(oil_token.location),
            certification: Set(oil_token.certification),
            quality_report: Set(oil_token.quality_report),
            storage_conditions: Set(oil_token.storage_conditions),
            expiry_date: Set(
                DateTime::parse_from_rfc3339(&oil_token.expiry_date)
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            current_price: Set(Decimal::from_f64(oil_token.current_price).unwrap()),
            reserve_price: Set(Decimal::from_f64(oil_token.reserve_price).unwrap()),
            status: Set(
                Some(match oil_token.status.as_str() {
                    "active" => TokenStatus::Active,
                    "pending" => TokenStatus::Pending,
                    "sold" => TokenStatus::Sold,
                    "expired" => TokenStatus::Expired,
                    "cancelled" => TokenStatus::Cancelled,
                    _ => TokenStatus::Active, // default/fallback
                })
            ),
            owner: Set(oil_token.owner),
            created_at: Set(
                DateTime::parse_from_rfc3339(&oil_token.created_at)
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            updated_at: Set(
                DateTime::parse_from_rfc3339(&oil_token.updated_at)
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            ..Default::default()
        };
        model.insert(db).await?;
    }
    Ok(())
}

async fn seed_tokenizations(db: &DatabaseConnection) -> Result<Vec<tokenization::Model>, DbErr> {
    let tokenizations_data = vec![
        tokenization::ActiveModel {
            id: Set(1),
            oil_token_id: Set(1),
            tokenizer_id: Set("0x1234567890abcdef1234567890abcdef12345678".to_string()),
            amount: Set(Decimal::from_f64(75000.0).unwrap()),
            created_at: Set(
                DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            updated_at: Set(
                DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            ..Default::default()
        },
        tokenization::ActiveModel {
            id: Set(2),
            oil_token_id: Set(2),
            tokenizer_id: Set("0x2345678901bcdef2345678901bcdef2345678901".to_string()),
            amount: Set(Decimal::from_f64(38000.0).unwrap()),
            created_at: Set(
                DateTime::parse_from_rfc3339("2024-01-02T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            updated_at: Set(
                DateTime::parse_from_rfc3339("2024-01-02T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            ..Default::default()
        },
        tokenization::ActiveModel {
            id: Set(3),
            oil_token_id: Set(3),
            tokenizer_id: Set("0x3456789012cdef3456789012cdef3456789012c".to_string()),
            amount: Set(Decimal::from_f64(8500.0).unwrap()),
            created_at: Set(
                DateTime::parse_from_rfc3339("2024-01-03T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            updated_at: Set(
                DateTime::parse_from_rfc3339("2024-01-03T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            ..Default::default()
        },
        tokenization::ActiveModel {
            id: Set(4),
            oil_token_id: Set(4),
            tokenizer_id: Set("0x4567890123def4567890123def4567890123def".to_string()),
            amount: Set(Decimal::from_f64(12000.0).unwrap()),
            created_at: Set(
                DateTime::parse_from_rfc3339("2024-01-04T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            updated_at: Set(
                DateTime::parse_from_rfc3339("2024-01-04T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            ..Default::default()
        },
        tokenization::ActiveModel {
            id: Set(5),
            oil_token_id: Set(5),
            tokenizer_id: Set("0x5678901234ef5678901234ef5678901234ef567".to_string()),
            amount: Set(Decimal::from_f64(25000.0).unwrap()),
            created_at: Set(
                DateTime::parse_from_rfc3339("2024-01-05T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            updated_at: Set(
                DateTime::parse_from_rfc3339("2024-01-05T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            ..Default::default()
        }
    ];

    let mut tokenizations = Vec::new();
    for tokenization_data in tokenizations_data {
        let tokenization = tokenization_data.insert(db).await?;
        tokenizations.push(tokenization);
    }

    Ok(tokenizations)
}

async fn seed_comments(db: &DatabaseConnection) -> Result<Vec<comment::Model>, DbErr> {
    let comments_data = vec![
        comment::ActiveModel {
            id: Set(1),
            oil_token_id: Set(1),
            user: Set("0x1234567890abcdef1234567890abcdef12345678".to_string()),
            content: Set(
                "High quality Brent crude with excellent specifications for refining.".to_string()
            ),
            created_at: Set(
                DateTime::parse_from_rfc3339("2024-01-01T10:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            updated_at: Set(
                DateTime::parse_from_rfc3339("2024-01-01T10:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            ..Default::default()
        },
        comment::ActiveModel {
            id: Set(2),
            oil_token_id: Set(2),
            user: Set("0x2345678901bcdef2345678901bcdef2345678901".to_string()),
            content: Set("WTI crude from Cushing hub - premium grade for US markets.".to_string()),
            created_at: Set(
                DateTime::parse_from_rfc3339("2024-01-02T11:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            updated_at: Set(
                DateTime::parse_from_rfc3339("2024-01-02T11:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            ..Default::default()
        },
        comment::ActiveModel {
            id: Set(3),
            oil_token_id: Set(3),
            user: Set("0x3456789012cdef3456789012cdef3456789012c".to_string()),
            content: Set("ULSD meets all European specifications for road transport.".to_string()),
            created_at: Set(
                DateTime::parse_from_rfc3339("2024-01-03T12:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            updated_at: Set(
                DateTime::parse_from_rfc3339("2024-01-03T12:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            ..Default::default()
        },
        comment::ActiveModel {
            id: Set(4),
            oil_token_id: Set(4),
            user: Set("0x4567890123def4567890123def4567890123def".to_string()),
            content: Set("Jet A-1 fuel certified for international aviation use.".to_string()),
            created_at: Set(
                DateTime::parse_from_rfc3339("2024-01-04T13:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            updated_at: Set(
                DateTime::parse_from_rfc3339("2024-01-04T13:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            ..Default::default()
        },
        comment::ActiveModel {
            id: Set(5),
            oil_token_id: Set(5),
            user: Set("0x5678901234ef5678901234ef5678901234ef567".to_string()),
            content: Set(
                "High-purity ethane from Permian Basin for petrochemical use.".to_string()
            ),
            created_at: Set(
                DateTime::parse_from_rfc3339("2024-01-05T14:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            updated_at: Set(
                DateTime::parse_from_rfc3339("2024-01-05T14:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            ..Default::default()
        },
        comment::ActiveModel {
            id: Set(6),
            oil_token_id: Set(1),
            user: Set("0x9876543210fedcba9876543210fedcba98765432".to_string()),
            content: Set(
                "Excellent API gravity and low sulfur content. Perfect for premium gasoline production.".to_string()
            ),
            created_at: Set(
                DateTime::parse_from_rfc3339("2024-01-01T15:30:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            updated_at: Set(
                DateTime::parse_from_rfc3339("2024-01-01T15:30:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            ..Default::default()
        },
        comment::ActiveModel {
            id: Set(7),
            oil_token_id: Set(2),
            user: Set("0x8765432109edcba8765432109edcba8765432109".to_string()),
            content: Set("WTI quality is consistently high. Great for US refineries.".to_string()),
            created_at: Set(
                DateTime::parse_from_rfc3339("2024-01-02T16:45:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            updated_at: Set(
                DateTime::parse_from_rfc3339("2024-01-02T16:45:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            ..Default::default()
        }
    ];

    let mut comments = Vec::new();
    for comment_data in comments_data {
        let comment = comment_data.insert(db).await?;
        comments.push(comment);
    }

    Ok(comments)
}

async fn seed_saved_tokens(db: &DatabaseConnection) -> Result<Vec<saved_token::Model>, DbErr> {
    let saved_tokens_data = vec![
        saved_token::ActiveModel {
            id: Set(1),
            user: Set("0x1234567890abcdef1234567890abcdef12345678".to_string()),
            oil_token_id: Set(1),
            created_at: Set(
                DateTime::parse_from_rfc3339("2024-01-01T15:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            ..Default::default()
        },
        saved_token::ActiveModel {
            id: Set(2),
            user: Set("0x2345678901bcdef2345678901bcdef2345678901".to_string()),
            oil_token_id: Set(2),
            created_at: Set(
                DateTime::parse_from_rfc3339("2024-01-02T16:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            ..Default::default()
        },
        saved_token::ActiveModel {
            id: Set(3),
            user: Set("0x3456789012cdef3456789012cdef3456789012c".to_string()),
            oil_token_id: Set(3),
            created_at: Set(
                DateTime::parse_from_rfc3339("2024-01-03T17:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            ..Default::default()
        },
        saved_token::ActiveModel {
            id: Set(4),
            user: Set("0x4567890123def4567890123def4567890123def".to_string()),
            oil_token_id: Set(4),
            created_at: Set(
                DateTime::parse_from_rfc3339("2024-01-04T18:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            ..Default::default()
        },
        saved_token::ActiveModel {
            id: Set(5),
            user: Set("0x5678901234ef5678901234ef5678901234ef567".to_string()),
            oil_token_id: Set(5),
            created_at: Set(
                DateTime::parse_from_rfc3339("2024-01-05T19:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            ..Default::default()
        },
        saved_token::ActiveModel {
            id: Set(6),
            user: Set("0x9876543210fedcba9876543210fedcba98765432".to_string()),
            oil_token_id: Set(1),
            created_at: Set(
                DateTime::parse_from_rfc3339("2024-01-01T20:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            ..Default::default()
        },
        saved_token::ActiveModel {
            id: Set(7),
            user: Set("0x8765432109edcba8765432109edcba8765432109".to_string()),
            oil_token_id: Set(2),
            created_at: Set(
                DateTime::parse_from_rfc3339("2024-01-02T21:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            ..Default::default()
        },
        saved_token::ActiveModel {
            id: Set(8),
            user: Set("0x7654321098dcba7654321098dcba7654321098".to_string()),
            oil_token_id: Set(3),
            created_at: Set(
                DateTime::parse_from_rfc3339("2024-01-03T22:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
                    .naive_utc()
            ),
            ..Default::default()
        }
    ];

    let mut saved_tokens = Vec::new();
    for saved_token_data in saved_tokens_data {
        let saved_token = saved_token_data.insert(db).await?;
        saved_tokens.push(saved_token);
    }

    Ok(saved_tokens)
}

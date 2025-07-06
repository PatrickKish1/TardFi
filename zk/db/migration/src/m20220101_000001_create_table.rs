use sea_orm_migration::{ prelude::*, schema::* };
use sea_orm::{ EnumIter, Iterable };
use sea_orm_migration::prelude::extension::postgres::Type;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum OilToken {
    Table,
    Id,
    TokenId,
    OilType,
    Grade,
    Quantity,
    Unit,
    Location,
    Certification,
    QualityReport,
    StorageConditions,
    ExpiryDate,
    CurrentPrice,
    ReservePrice,
    Status,
    Owner,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum Tokenization {
    Table,
    Id,
    OilTokenId,
    TokenizerId,
    Amount,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum Comment {
    Table,
    Id,
    OilTokenId,
    User,
    Content,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum SavedToken {
    Table,
    Id,
    User,
    OilTokenId,
    CreatedAt,
}

#[derive(EnumIter)]
pub enum TokenStatus {
    Pending,
    Active,
    Sold,
    Expired,
    Cancelled,
}

impl TokenStatus {
    fn as_str(&self) -> &'static str {
        match self {
            TokenStatus::Pending => "pending",
            TokenStatus::Active => "active",
            TokenStatus::Sold => "sold",
            TokenStatus::Expired => "expired",
            TokenStatus::Cancelled => "cancelled",
        }
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the token status enum type
        manager.create_type(
            Type::create()
                .as_enum("token_status")
                .values(TokenStatus::iter().map(|v| v.as_str()))
                .to_owned()
        ).await?;

        // Create oil_token table
        manager.create_table(
            Table::create()
                .table(OilToken::Table)
                .if_not_exists()
                .col(pk_auto(OilToken::Id))
                .col(integer(OilToken::TokenId))
                .col(string(OilToken::OilType))
                .col(string(OilToken::Grade))
                .col(decimal(OilToken::Quantity))
                .col(string(OilToken::Unit))
                .col(string(OilToken::Location))
                .col(string(OilToken::Certification))
                .col(json(OilToken::QualityReport))
                .col(json(OilToken::StorageConditions))
                .col(timestamp(OilToken::ExpiryDate))
                .col(decimal(OilToken::CurrentPrice))
                .col(decimal(OilToken::ReservePrice))
                .col(
                    enumeration_null(OilToken::Status, "token_status", [
                        "pending",
                        "active",
                        "sold",
                        "expired",
                        "cancelled",
                    ])
                )
                .col(string(OilToken::Owner))
                .col(timestamp(OilToken::CreatedAt))
                .col(timestamp(OilToken::UpdatedAt))
                .to_owned()
        ).await?;

        // Create tokenization table (replaces auction table)
        manager.create_table(
            Table::create()
                .table(Tokenization::Table)
                .if_not_exists()
                .col(pk_auto(Tokenization::Id))
                .col(integer(Tokenization::OilTokenId))
                .col(string(Tokenization::TokenizerId))
                .col(decimal(Tokenization::Amount))
                .col(timestamp(Tokenization::CreatedAt))
                .col(timestamp(Tokenization::UpdatedAt))
                .to_owned()
        ).await?;

        // Create comment table
        manager.create_table(
            Table::create()
                .table(Comment::Table)
                .if_not_exists()
                .col(pk_auto(Comment::Id))
                .col(integer(Comment::OilTokenId))
                .col(string(Comment::User))
                .col(text(Comment::Content))
                .col(timestamp(Comment::CreatedAt))
                .col(timestamp(Comment::UpdatedAt))
                .to_owned()
        ).await?;

        // Create saved_token table (replaces saved_auction)
        manager.create_table(
            Table::create()
                .table(SavedToken::Table)
                .if_not_exists()
                .col(pk_auto(SavedToken::Id))
                .col(string(SavedToken::User))
                .col(integer(SavedToken::OilTokenId))
                .col(timestamp(SavedToken::CreatedAt))
                .to_owned()
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order
        manager.drop_table(Table::drop().table(SavedToken::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Comment::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Tokenization::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(OilToken::Table).to_owned()).await?;

        // Drop the enum type
        manager.drop_type(Type::drop().name("token_status").to_owned()).await?;

        Ok(())
    }
}

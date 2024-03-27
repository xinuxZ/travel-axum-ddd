use serde::Deserialize;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;

pub mod repository;
pub mod service;

pub use service::*;
pub use repository::*;

#[derive(FromRow, Deserialize)]
pub struct ArticleEntity {
    pub id: i64,

    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,

    #[serde(with = "time::serde::iso8601")]
    pub updated_at: OffsetDateTime,

    pub title: String,
    pub body: String,
    pub description: String,
    pub slug: String,
    pub user_id: i64,
    pub favorites: i64,
    pub favorited: bool,
    pub following_author: bool,
    pub author_username: String,
    pub author_image: String,
    pub author_bio: String,
}

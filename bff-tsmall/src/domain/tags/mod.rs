pub mod repository;
pub mod service;

pub use service::*;
pub use repository::*;

use sqlx::types::time::OffsetDateTime;

pub struct TagEntity {
    pub id: i64,
    pub tag: String,
    pub created_at: OffsetDateTime,
}

pub struct ArticleTagEntity {
    pub id: i64,
    pub tag_id: i64,
    pub article_id: i64,
    pub created_at: OffsetDateTime,
}

pub struct ArticleTagQuery {
    pub id: i64,
    pub tag_id: i64,
    pub article_id: i64,
    pub tag: String,
}

impl From<TagEntity> for String {
    fn from(entity: TagEntity) -> Self {
        entity.tag
    }
}

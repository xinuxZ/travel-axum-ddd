use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;
// use time::format_description;

use crate::application::articles::AuthorDto;
use crate::application::comments::CommentDto;

pub mod repository;
pub mod service;

pub use service::*;
pub use repository::*;

#[derive(FromRow)]
pub struct CommentEntity {
    pub id: i64,
    pub body: String,
    pub user_id: i64,
    pub article_id: i64,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(FromRow)]
pub struct CommentQuery {
    pub id: i64,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub body: String,
    pub author_username: String,
    pub author_bio: String,
    pub author_image: String,
    pub following_author: bool,
}

impl From<CommentQuery> for CommentDto {
    fn from(query: CommentQuery) -> Self {
        // let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]");
        Self {
            id: query.id,
            created_at: query.created_at.to_string(),
            updated_at: query.updated_at.to_string(),
            body: query.body,
            author: AuthorDto {
                username: query.author_username,
                bio: Some(query.author_bio),
                image: Some(query.author_image),
                following: query.following_author,
            },
        }
    }
}

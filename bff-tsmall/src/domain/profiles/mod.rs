pub mod repository;
pub mod service;

pub use service::*;
pub use repository::*;

use sqlx::types::time::OffsetDateTime;
use std::time::SystemTime;

pub struct UserFollowEntity {
    pub id: i64,
    pub created_at: OffsetDateTime,
    pub follower_id: i64,
    pub followee_id: i64,
}

impl UserFollowEntity {
    pub fn default() -> Self {
        Self {
            id: 1,
            follower_id: 2,
            followee_id: 1,
            created_at: OffsetDateTime::from(SystemTime::now()),
        }
    }
}

// use chrono::{DateTime, Local};
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;
use std::time::SystemTime;

use crate::application::profiles::ProfileDto;
use crate::application::users::UserDto;

pub mod repository;
pub mod service;

pub use repository::*;
pub use service::*;

#[derive(FromRow)]
pub struct UserEntity {
    pub id: i64,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    // pub created_at: DateTime<Local>,
    // pub updated_at: DateTime<Local>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: String,
    pub image: String,
}

impl UserEntity {
    pub fn into_dto(self, token: String) -> UserDto {
        UserDto {
            id: self.id,
            email: self.email,
            username: self.username,
            bio: Some(self.bio),
            image: Some(self.image),
            token,
        }
    }

    pub fn into_profile(self, following: bool) -> ProfileDto {
        ProfileDto {
            username: self.username,
            bio: self.bio,
            image: self.image,
            following,
        }
    }
}

impl Default for UserEntity {
    fn default() -> Self {
        UserEntity {
            id: 1,
            bio: String::from("stub bio"),
            created_at: OffsetDateTime::from(SystemTime::now()),
            updated_at: OffsetDateTime::from(SystemTime::now()),
            username: String::from("stub username"),
            email: String::from("stub email"),
            password: String::from("hashed password"),
            image: String::from("stub image"),
        }
    }
}

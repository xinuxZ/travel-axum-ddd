use serde::{Deserialize, Serialize};

use crate::domain::UserEntity;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserDto {
    #[serde(skip_serializing, skip_deserializing)]
    pub id: i64,
    pub username: String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub token: String,
}

impl UserDto {
    pub fn from(user: UserEntity) -> Self {
        UserDto {
            id: user.id,
            username: user.username,
            email: user.email,
            bio: Some(user.bio),
            image: Some(user.image),
            token: String::new(),
        }
    }
}

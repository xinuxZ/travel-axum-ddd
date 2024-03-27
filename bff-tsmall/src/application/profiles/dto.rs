use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ProfileDto {
    pub username: String,
    pub bio: String,
    pub image: String,
    pub following: bool,
}

use serde::{Deserialize, Serialize};

use crate::application::articles::AuthorDto;

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct CommentDto {
    pub id: i64,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub body: String,
    pub author: AuthorDto,
}

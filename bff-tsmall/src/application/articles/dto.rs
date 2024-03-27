use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct ArticleDto {
    #[serde(skip_serializing, skip_deserializing)]
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    #[serde(rename = "tagList")]
    pub tag_list: Vec<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub favorited: bool,
    #[serde(rename = "favoritesCount")]
    pub favorites_count: i64,
    pub author: AuthorDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AuthorDto {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}

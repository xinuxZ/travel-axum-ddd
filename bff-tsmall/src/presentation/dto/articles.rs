use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::application::articles::{CreateArticleCommand, UpdateArticleCommand};
use crate::application::articles::ArticleDto;

lazy_static! {
    pub static ref LIMIT: i64 = 20;
    pub static ref OFFSET: i64 = 0;
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateArticleRequest {
    #[validate]
    pub article: CreateArticleCommand,
}

#[derive(Debug, Deserialize)]
pub struct UpdateArticleRequest {
    pub article: UpdateArticleCommand,
}

#[derive(Debug, Deserialize)]
pub struct GetArticlesApiRequest {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug)]
pub struct GetArticlesServiceRequest {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub limit: i64,
    pub offset: i64,
}

impl From<GetArticlesApiRequest> for GetArticlesServiceRequest {
    fn from(request: GetArticlesApiRequest) -> Self {
        Self {
            tag: request.tag,
            author: request.author,
            favorited: request.favorited,
            limit: request.limit.unwrap_or_else(|| LIMIT.abs()),
            offset: request.offset.unwrap_or_else(|| OFFSET.abs()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ArticlesResponse {
    pub articles: Vec<ArticleDto>,
    #[serde(rename = "articlesCount")]
    pub articles_count: usize,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ArticleResponse {
    pub article: ArticleDto,
}

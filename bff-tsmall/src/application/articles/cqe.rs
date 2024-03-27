use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct GetArticlesQuery {
    pub user_id: Option<i64>,

    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,

    // #[validate(required, range(min = 5, max = 50, message = "每页最多50条"))]
    pub limit: i64,

    // #[validate(required)]
    pub offset: i64,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateArticleCommand {
    #[validate(required, length(min = 1))]
    pub title: Option<String>,

    #[validate(required, length(min = 1))]
    pub description: Option<String>,

    #[validate(required, length(min = 1))]
    pub body: Option<String>,

    #[serde(rename = "tagList")]
    pub tag_list: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateArticleCommand {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}

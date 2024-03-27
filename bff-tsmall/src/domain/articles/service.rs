use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;

use crate::application::articles::GetArticlesQuery;
use crate::application::articles::ArticleDto;
use crate::infrastructure::errors::AppResult;

pub type DynArticlesService = Arc<dyn ArticlesService + Send + Sync>;

#[automock]
#[async_trait]
pub trait ArticlesService {
    async fn create_article(
        &self,
        user_id: i64,
        title: String,
        description: String,
        body: String,
        tag_list: Vec<String>,
    ) -> AppResult<ArticleDto>;

    async fn update_article(
        &self,
        user_id: i64,
        slug: String,
        title: Option<String>,
        description: Option<String>,
        body: Option<String>,
    ) -> AppResult<ArticleDto>;

    async fn get_articles(&self, query: GetArticlesQuery) -> AppResult<Vec<ArticleDto>>;

    async fn get_article(&self, user_id: Option<i64>, slug: String) -> AppResult<ArticleDto>;

    async fn get_feed(&self, user_id: i64, limit: i64, offset: i64) -> AppResult<Vec<ArticleDto>>;

    async fn delete_article(&self, user_id: i64, slug: String) -> AppResult<()>;

    async fn favorite_article(&self, user_id: i64, slug: String) -> AppResult<ArticleDto>;

    async fn unfavorite_article(&self, user_id: i64, slug: String) -> AppResult<ArticleDto>;
}

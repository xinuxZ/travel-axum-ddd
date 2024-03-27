use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;

use crate::domain::tags::{ArticleTagQuery, TagEntity};
pub type DynTagsRepository = Arc<dyn TagsRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait TagsRepository {
    async fn get_tags(&self, tags: Vec<String>) -> anyhow::Result<Vec<TagEntity>>;

    async fn create_tags(&self, tags: Vec<String>) -> anyhow::Result<Vec<TagEntity>>;

    async fn get_article_tags_by_article_id(&self, article_id: i64) -> anyhow::Result<Vec<ArticleTagQuery>>;

    async fn get_article_tags_article_ids(&self, article_ids: Vec<i64>) -> anyhow::Result<Vec<ArticleTagQuery>>;

    async fn create_article_tags(&self, tags: Vec<(i64, i64)>) -> anyhow::Result<()>;
}

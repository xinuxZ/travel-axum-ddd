use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;

use crate::domain::comments::CommentEntity;
use crate::domain::comments::CommentQuery;

pub type DynCommentsRepository = Arc<dyn CommentsRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait CommentsRepository {
    async fn get_comments(&self, user_id: Option<i64>, article_id: i64) -> anyhow::Result<Vec<CommentQuery>>;

    async fn get_comment(&self, comment_id: i64) -> anyhow::Result<Option<CommentEntity>>;

    async fn create_comment(&self, article_id: i64, user_id: i64, body: String) -> anyhow::Result<CommentQuery>;

    async fn delete_comment(&self, comment_id: i64) -> anyhow::Result<()>;
}

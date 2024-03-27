use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;

use crate::application::comments::CommentDto;

use crate::infrastructure::errors::AppResult;

pub type DynCommentsService = Arc<dyn CommentsService + Send + Sync>;

#[automock]
#[async_trait]
pub trait CommentsService {
    async fn get_comments(&self, user_id: Option<i64>, slug: String) -> AppResult<Vec<CommentDto>>;

    async fn add_comment(&self, user_id: i64, slug: String, body: String) -> AppResult<CommentDto>;

    async fn remove_comment(&self, user_id: i64, comment_id: i64) -> AppResult<()>;
}

use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;

use crate::infrastructure::errors::AppResult;

pub type DynTagsService = Arc<dyn TagsService + Send + Sync>;

#[automock]
#[async_trait]
pub trait TagsService {
    async fn get_tags(&self) -> AppResult<Vec<String>>;
}

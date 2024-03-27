use async_trait::async_trait;
use itertools::Itertools;
use tracing::info;

use crate::domain::tags::DynTagsRepository;
use crate::domain::tags::TagsService;
use crate::infrastructure::errors::AppResult;

pub struct AppTagsService {
    tags_repository: DynTagsRepository,
}

impl AppTagsService {
    pub fn new(tags_repository: DynTagsRepository) -> Self {
        Self { tags_repository }
    }
}

#[async_trait]
impl TagsService for AppTagsService {
    async fn get_tags(&self) -> AppResult<Vec<String>> {
        let tags = self
            .tags_repository
            .get_tags(vec![])
            .await?
            .into_iter()
            .map_into::<String>()
            .collect_vec();

        info!("found {:?} tags", tags.len());

        Ok(tags)
    }
}

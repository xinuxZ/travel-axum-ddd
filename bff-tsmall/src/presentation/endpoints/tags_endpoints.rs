use axum::{Extension, Json};
use tracing::info;

use crate::domain::tags::DynTagsService;
use crate::infrastructure::errors::AppResult;

use crate::presentation::dto::tags::TagsResponse;

pub async fn get_tags(Extension(tags_service): Extension<DynTagsService>) -> AppResult<Json<TagsResponse>> {
    info!("recieved request to retrieve all tags");

    let tags = tags_service.get_tags().await?;

    Ok(Json(TagsResponse { tags }))
}

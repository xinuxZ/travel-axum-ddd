use std::collections::HashMap;

use axum::extract::Path;
use axum::{Extension, Json};
use tracing::info;

use crate::domain::profiles::DynProfilesService;
use crate::infrastructure::errors::AppResult;

use crate::presentation::dto::profiles::ProfileResponse;
use crate::presentation::extractors::optional_authentication_extractor::OptionalAuthentication;
use crate::presentation::extractors::required_authentication_extractor::RequiredAuthentication;

pub async fn get_profile(
    Path(params): Path<HashMap<String, String>>,
    Extension(profiles_service): Extension<DynProfilesService>,
    OptionalAuthentication(user_id): OptionalAuthentication,
) -> AppResult<Json<ProfileResponse>> {
    let username = params.get("username").unwrap();

    info!("recieved request to get profile {:?}", username);

    let profile = profiles_service.get_profile(username, user_id).await?;

    Ok(Json(ProfileResponse { profile }))
}

pub async fn follow_user(
    Path(params): Path<HashMap<String, String>>,
    Extension(profiles_service): Extension<DynProfilesService>,
    RequiredAuthentication(user_id): RequiredAuthentication,
) -> AppResult<Json<ProfileResponse>> {
    let username = params.get("username").unwrap();

    info!(
        "recieved request to follow profile {:?} from user ID {:?}",
        username, user_id
    );

    let profile = profiles_service.add_user_follow(username, user_id).await?;

    Ok(Json(ProfileResponse { profile }))
}

pub async fn unfollow_user(
    Path(params): Path<HashMap<String, String>>,
    Extension(profiles_service): Extension<DynProfilesService>,
    RequiredAuthentication(user_id): RequiredAuthentication,
) -> AppResult<Json<ProfileResponse>> {
    let username = params.get("username").unwrap();

    info!(
        "recieved request to unfollow profile {:?} from user ID {:?}",
        username, user_id
    );

    let profile = profiles_service.remove_user_follow(username, user_id).await?;

    Ok(Json(ProfileResponse { profile }))
}

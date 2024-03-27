use axum::{extract::Path, extract::Query, Extension, Json};
use tracing::info;

use crate::application::users::PageUsersQuery;
use crate::domain::users::DynUsersService;
use crate::infrastructure::errors::AppResult;

use crate::presentation::dto::users::{
    LoginUserRequest, PageUsersRequest, RegisterUserRequest, UpdateUserRequest, UserAuthenicationResponse,
    UsersResponse, LIMIT, OFFSET,
};

use crate::presentation::extractors::{
    required_authentication_extractor::RequiredAuthentication, validation_extractor::ValidationExtractor,
};

pub async fn users_endpoints(
    query: Query<PageUsersRequest>,
    Extension(users_service): Extension<DynUsersService>,
) -> AppResult<Json<UsersResponse>> {
    info!("recieved request to query users &offset={:?}", query);

    let page_users_query = PageUsersQuery {
        limit: query.0.limit.unwrap_or_else(|| LIMIT.abs()),
        offset: query.0.offset.unwrap_or_else(|| OFFSET.abs()),
    };

    let users = users_service.page_users(page_users_query).await?;
    let users_count = users.len();

    Ok(Json(UsersResponse { users, users_count }))
}

pub async fn register_user_endpoint(
    Extension(users_service): Extension<DynUsersService>,
    ValidationExtractor(request): ValidationExtractor<RegisterUserRequest>,
) -> AppResult<Json<UserAuthenicationResponse>> {
    info!(
        "recieved request to create user {:?}/{:?}",
        request.user.email.as_ref().unwrap(),
        request.user.username.as_ref().unwrap()
    );

    let created_user = users_service.register_user(request.user).await?;

    Ok(Json(UserAuthenicationResponse { user: created_user }))
}

pub async fn login_user_endpoint(
    Extension(users_service): Extension<DynUsersService>,
    ValidationExtractor(request): ValidationExtractor<LoginUserRequest>,
) -> AppResult<Json<UserAuthenicationResponse>> {
    info!(
        "recieved request to login user {:?}",
        request.user.email.as_ref().unwrap()
    );

    let created_user = users_service.login_user(request.user).await?;

    Ok(Json(UserAuthenicationResponse { user: created_user }))
}

pub async fn get_current_user_endpoint(
    RequiredAuthentication(user_id): RequiredAuthentication,
    // Path(user_id): Path<i64>,
    Extension(users_service): Extension<DynUsersService>,
) -> AppResult<Json<UserAuthenicationResponse>> {
    info!("recieved request to retrieve current user");

    let current_user = users_service.get_current_user(user_id).await?;

    Ok(Json(UserAuthenicationResponse { user: current_user }))
}

pub async fn update_user_endpoint(
    RequiredAuthentication(user_id): RequiredAuthentication,
    Extension(users_service): Extension<DynUsersService>,
    Json(request): Json<UpdateUserRequest>,
) -> AppResult<Json<UserAuthenicationResponse>> {
    info!("recieved request to update user {:?}", user_id);

    let updated_user = users_service.updated_user(user_id, request.user).await?;

    Ok(Json(UserAuthenicationResponse { user: updated_user }))
}

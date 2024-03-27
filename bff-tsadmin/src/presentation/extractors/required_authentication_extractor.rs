use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::{header::AUTHORIZATION, request::Parts};
use axum::Extension;
use tracing::error;

use crate::domain::utils::DynTokenService;
use crate::infrastructure::app_errors::AppError;

/// Extracts the JWT from the Authorization token header.
pub struct RequiredAuthentication(pub i64);

#[async_trait]
impl<S> FromRequestParts<S> for RequiredAuthentication
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Extension(token_service): Extension<DynTokenService> = Extension::from_request_parts(parts, state)
            .await
            .map_err(|err| AppError::InternalServerErrorWithContext(err.to_string()))?;

        if let Some(authorization_header) = parts.headers.get(AUTHORIZATION) {
            let header_value = authorization_header.to_str().map_err(|_| AppError::Unauthorized)?;

            if !header_value.contains("Token") {
                error!("request does not contain valid 'Token' prefix for authorization");
                return Err(AppError::Unauthorized);
            }

            let tokenized_value: Vec<_> = header_value.split(' ').collect();

            if tokenized_value.len() != 2 || tokenized_value.get(1).is_none() {
                error!("request does not contain a valid token");
                return Err(AppError::Unauthorized);
            }

            let token_value = tokenized_value.into_iter().nth(1).unwrap();
            let user_id = token_service
                .get_user_id_from_token(String::from(token_value))
                .map_err(|err| {
                    error!("could not validate user ID from token: {:?}", err);
                    AppError::Unauthorized
                })?;

            Ok(RequiredAuthentication(user_id))
        } else {
            Err(AppError::Unauthorized)
        }
    }
}

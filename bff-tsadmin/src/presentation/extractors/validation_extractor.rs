use axum::async_trait;
use axum::extract::FromRequest;
use axum::extract::Request;
// use axum::BoxError;
use axum::Json;
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::infrastructure::app_errors::AppError;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidationExtractor<T>(pub T);

// #[async_trait]
// impl<S, B, T> FromRequest<S, B> for ValidationExtractor<T>
// where
//     // these bounds are required by `async_trait`
//     B: Send + 'static + axum::body::HttpBody,
//     S: Send + Sync,
//     T: DeserializeOwned + Validate,
//     B::Data: Send,
//     B::Error: Into<BoxError>,
// {
//     type Rejection = AppError;

//     async fn from_request(request: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
//         let Json(value) = Json::<T>::from_request(request, state).await?;
//         value.validate()?;
//         Ok(ValidationExtractor(value))
//     }
// }

#[async_trait]
impl<S, T> FromRequest<S> for ValidationExtractor<T>
where
    // these bounds are required by `async_trait`
    S: Send + Sync,
    T: FromRequest<S>,
    T: DeserializeOwned + Validate,
{
    type Rejection = AppError;

    async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(request, state).await?;
        value.validate()?;
        Ok(ValidationExtractor(value))
    }
}

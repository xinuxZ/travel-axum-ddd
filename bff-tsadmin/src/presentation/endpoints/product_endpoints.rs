use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use axum_macros::debug_handler;
use tracing::info;

use crate::application::product::ProductAppService;
use crate::infrastructure::app_errors::AppResult;
use crate::presentation::dto::product::{ProductRequest, ProductResponse, ProductsResponse};

pub async fn product(
    Path(id): Path<i64>,
    Extension(product_service): Extension<ProductAppService>,
) -> AppResult<Json<ProductResponse>> {
    info!("recived request to query product {:?}", id);

    let product = product_service.get(id).await?;

    Ok(Json(ProductResponse { product }))
}

#[debug_handler]
pub async fn products(
    Query(query): Query<ProductRequest>,
    Extension(product_service): Extension<ProductAppService>,
) -> AppResult<Json<ProductsResponse>> {
    info!("recived request to query product {:?}", &query);

    // 转换为 appservice 层的cqe格式
    let product_query = query.to_cqe();

    let products = product_service.list(product_query).await?;

    Ok(Json(ProductsResponse { products }))
}

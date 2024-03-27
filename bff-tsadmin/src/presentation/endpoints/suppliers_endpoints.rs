use axum::{extract::Path, Extension, Json};
use tracing::info;

use crate::application::suppliers::SupplierAppService;
use crate::infrastructure::app_errors::AppResult;
use crate::presentation::dto::suppliers::SupplierResponse;

pub async fn supplier(
    Path(id): Path<i64>,
    Extension(suppliers_service): Extension<SupplierAppService>,
) -> AppResult<Json<SupplierResponse>> {
    info!("recived request to query supplier {:?}", id);

    let supplier = suppliers_service.get(id).await?;

    Ok(Json(SupplierResponse { supplier }))
}

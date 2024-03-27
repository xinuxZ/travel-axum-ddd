use axum::routing::get;
use axum::{Extension, Router};

use crate::application::suppliers::SupplierAppService;
use crate::presentation::endpoints::suppliers_endpoints::supplier;

pub struct SuppliersRouter;

impl SuppliersRouter {
    pub fn new_router(suppliers_service: SupplierAppService) -> Router {
        Router::new()
            .route("/supplier/:id", get(supplier))
            .layer(Extension(suppliers_service))
    }
}

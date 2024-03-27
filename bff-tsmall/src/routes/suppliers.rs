use axum::routing::{delete, get, post, put};
use axum::{Extension, Router};

use crate::presentation::endpoints::suppliers_endpoints::supplier;
use crate::register::ServiceRegister;

pub struct SuppliersRouter;

impl SuppliersRouter {
    pub fn new_router(service_register: ServiceRegister) -> Router {
        Router::new()
            .route("/supplier/:id", get(supplier))
            .layer(Extension(service_register.suppliers_service))
    }
}

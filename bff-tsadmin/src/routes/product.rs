use axum::routing::get;
use axum::{Extension, Router};

use crate::application::product::ProductAppService;
use crate::presentation::endpoints::product_endpoints::{product, products};

pub struct ProductRouter;

impl ProductRouter {
    pub fn new_router(product_service: ProductAppService) -> Router {
        Router::new()
            .route("/product/:id", get(product))
            .route("/products", get(products))
            .layer(Extension(product_service))
    }
}

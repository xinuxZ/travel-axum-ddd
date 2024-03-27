use axum::routing::{delete, get, post, put};
use axum::{Extension, Router};

use crate::presentation::endpoints::tags_endpoints::get_tags;

use crate::register::ServiceRegister;

pub struct TagsRouter;

impl TagsRouter {
    pub fn new_router(service_register: ServiceRegister) -> Router {
        Router::new()
            .route("/tags", get(get_tags))
            .layer(Extension(service_register.tags_service))
    }
}

use axum::routing::{delete, get, post, put};
use axum::{Extension, Router};

use crate::presentation::endpoints::users_endpoints::{
    get_current_user_endpoint, login_user_endpoint, register_user_endpoint, update_user_endpoint, users_endpoints,
};

use crate::register::ServiceRegister;

pub struct UsersRouter;

impl UsersRouter {
    pub fn new_router(service_register: ServiceRegister) -> Router {
        Router::new()
            .route("/users", post(register_user_endpoint))
            .route("/users/login", post(login_user_endpoint))
            // .route("/user/:user_id", get(get_current_user_endpoint))
            .route("/user", get(get_current_user_endpoint))
            .route("/user", put(update_user_endpoint))
            .route("/users", get(users_endpoints))
            .layer(Extension(service_register.users_service))
            .layer(Extension(service_register.token_service))
    }
}

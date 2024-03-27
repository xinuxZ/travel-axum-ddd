use axum::routing::{delete, get, post, put};
use axum::{Extension, Router};

use crate::presentation::endpoints::profiles_endpoints::{follow_user, get_profile, unfollow_user};
use crate::register::ServiceRegister;

pub struct ProfilesRouter;

impl ProfilesRouter {
    pub fn new_router(service_register: ServiceRegister) -> Router {
        Router::new()
            .route("/profiles/:username", get(get_profile))
            .route("/profiles/:username/follow", post(follow_user))
            .route("/profiles/:username/follow", delete(unfollow_user))
            .layer(Extension(service_register.profiles_service))
            .layer(Extension(service_register.token_service))
    }
}

use axum::routing::{delete, get, post, put};
use axum::{Extension, Router};

use crate::presentation::endpoints::articles_endpoints::{
    add_comment, create_article, delete_article, favorite_article, get_article, get_article_feed, get_articles,
    get_comments, remove_comment, unfavorite_article, update_article,
};

use crate::presentation::endpoints::suppliers_endpoints::supplier;
use crate::presentation::endpoints::{
    profiles_endpoints::{follow_user, get_profile, unfollow_user},
    tags_endpoints::get_tags,
    users_endpoints::{
        get_current_user_endpoint, login_user_endpoint, register_user_endpoint, update_user_endpoint, users_endpoints,
    },
};

use crate::register::ServiceRegister;

use crate::application::suppliers::AppSupplierService;
use crate::domain::suppliers::repository::DynSuppliersRepository;
use crate::infrastructure::mysql::connection_pool::MySqlConnectionPool;
use crate::infrastructure::mysql::suppliers::MySqlSupplierRepository;
use std::sync::Arc;

pub struct ProfilesRouter;
pub struct UsersRouter;
pub struct ArticlesRouter;
pub struct TagsRouter;
pub struct SuppliersRouter;

impl SuppliersRouter {
    pub fn new_router(mysql: MySqlConnectionPool) -> Router {
        let suppliers_repository = Arc::new(MySqlSupplierRepository::new(mysql)) as DynSuppliersRepository;
        let suppliers_service = AppSupplierService::new(suppliers_repository.clone());

        Router::new()
            .route("/supplier/:id", get(supplier))
            .layer(Extension(suppliers_service))
    }
}

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

impl ArticlesRouter {
    pub fn new_router(service_register: ServiceRegister) -> Router {
        Router::new()
            .route("/articles", get(get_articles))
            .route("/articles", post(create_article))
            .route("/articles/feed", get(get_article_feed))
            .route("/articles/:slug", get(get_article))
            .route("/articles/:slug", put(update_article))
            .route("/articles/:slug", delete(delete_article))
            .route("/articles/:slug/favorite", post(favorite_article))
            .route("/articles/:slug/favorite", delete(unfavorite_article))
            .route("/articles/:slug/comments", get(get_comments))
            .route("/articles/:slug/comments", post(add_comment))
            .route("/articles/:slug/comments/:id", delete(remove_comment))
            .layer(Extension(service_register.articles_service))
            .layer(Extension(service_register.comments_service))
            .layer(Extension(service_register.token_service))
    }
}
impl TagsRouter {
    pub fn new_router(service_register: ServiceRegister) -> Router {
        Router::new()
            .route("/tags", get(get_tags))
            .layer(Extension(service_register.tags_service))
    }
}

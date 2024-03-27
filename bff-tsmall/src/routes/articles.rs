use axum::routing::{delete, get, post, put};
use axum::{Extension, Router};

use crate::presentation::endpoints::articles_endpoints::{
    add_comment, create_article, delete_article, favorite_article, get_article, get_article_feed, get_articles,
    get_comments, remove_comment, unfavorite_article, update_article,
};

use crate::register::ServiceRegister;

pub struct ArticlesRouter;

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

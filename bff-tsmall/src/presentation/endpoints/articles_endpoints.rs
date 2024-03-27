use axum::extract::{Path, Query};
use axum::{Extension, Json};
use tracing::info;

use crate::domain::articles::DynArticlesService;
use crate::domain::comments::DynCommentsService;
use crate::infrastructure::errors::AppResult;

use crate::application::articles::GetArticlesQuery;
use crate::presentation::dto::articles::{
    ArticleResponse, ArticlesResponse, CreateArticleRequest, GetArticlesApiRequest, UpdateArticleRequest, LIMIT, OFFSET,
};
use crate::presentation::dto::comments::{CommentResponse, CommentsResponse, CreateCommentRequest};

use crate::presentation::extractors::optional_authentication_extractor::OptionalAuthentication;
use crate::presentation::extractors::required_authentication_extractor::RequiredAuthentication;
use crate::presentation::extractors::validation_extractor::ValidationExtractor;

pub async fn get_articles(
    query_params: Query<GetArticlesApiRequest>,
    Extension(articles_service): Extension<DynArticlesService>,
    OptionalAuthentication(user_id): OptionalAuthentication,
) -> AppResult<Json<ArticlesResponse>> {
    info!("recieved request to retrieve articles {:?}", query_params.0);

    let get_articles_query = GetArticlesQuery {
        user_id: user_id,
        tag: query_params.0.tag,
        author: query_params.0.author,
        favorited: query_params.0.favorited,
        limit: query_params.0.limit.unwrap_or_else(|| LIMIT.abs()),
        offset: query_params.0.offset.unwrap_or_else(|| OFFSET.abs()),
    };

    let articles = articles_service.get_articles(get_articles_query).await?;

    let articles_count = articles.len();

    Ok(Json(ArticlesResponse {
        articles,
        articles_count,
    }))
}

pub async fn get_article_feed(
    query_params: Query<GetArticlesApiRequest>,
    Extension(articles_service): Extension<DynArticlesService>,
    RequiredAuthentication(user_id): RequiredAuthentication,
) -> AppResult<Json<ArticlesResponse>> {
    info!("recieved request to retrieve article feed for user {:?}", user_id);

    let articles = articles_service
        .get_feed(
            user_id,
            query_params.0.limit.unwrap_or_else(|| LIMIT.abs()),
            query_params.0.offset.unwrap_or_else(|| OFFSET.abs()),
        )
        .await?;

    let articles_count = articles.len();

    Ok(Json(ArticlesResponse {
        articles,
        articles_count,
    }))
}

pub async fn get_article(
    Path(slug): Path<String>,
    Extension(articles_service): Extension<DynArticlesService>,
    OptionalAuthentication(user_id): OptionalAuthentication,
) -> AppResult<Json<ArticleResponse>> {
    info!("recieved request to retrieve article {:?}", slug);

    let article = articles_service.get_article(user_id, slug).await?;

    Ok(Json(ArticleResponse { article }))
}

pub async fn create_article(
    Extension(articles_service): Extension<DynArticlesService>,
    RequiredAuthentication(user_id): RequiredAuthentication,
    ValidationExtractor(request): ValidationExtractor<CreateArticleRequest>,
) -> AppResult<Json<ArticleResponse>> {
    info!("recieved request to create article {:?}", request.article);

    let article = articles_service
        .create_article(
            user_id,
            request.article.title.unwrap(),
            request.article.description.unwrap(),
            request.article.body.unwrap(),
            request.article.tag_list,
        )
        .await?;

    Ok(Json(ArticleResponse { article }))
}

pub async fn update_article(
    Path(slug): Path<String>,
    Extension(articles_service): Extension<DynArticlesService>,
    RequiredAuthentication(user_id): RequiredAuthentication,
    Json(request): Json<UpdateArticleRequest>,
) -> AppResult<Json<ArticleResponse>> {
    info!("recieved request to update article {:?}", request.article);

    let article = articles_service
        .update_article(
            user_id,
            slug,
            request.article.title,
            request.article.description,
            request.article.body,
        )
        .await?;

    Ok(Json(ArticleResponse { article }))
}

pub async fn delete_article(
    Path(slug): Path<String>,
    Extension(articles_service): Extension<DynArticlesService>,
    RequiredAuthentication(user_id): RequiredAuthentication,
) -> AppResult<()> {
    info!("recieved request to delete article {:?}", slug);

    articles_service.delete_article(user_id, slug).await?;

    Ok(())
}

pub async fn favorite_article(
    Path(slug): Path<String>,
    Extension(articles_service): Extension<DynArticlesService>,
    RequiredAuthentication(user_id): RequiredAuthentication,
) -> AppResult<Json<ArticleResponse>> {
    info!("recieved request to favorite article {:?}", slug);

    let article = articles_service.favorite_article(user_id, slug).await?;

    Ok(Json(ArticleResponse { article }))
}

pub async fn unfavorite_article(
    Path(slug): Path<String>,
    Extension(articles_service): Extension<DynArticlesService>,
    RequiredAuthentication(user_id): RequiredAuthentication,
) -> AppResult<Json<ArticleResponse>> {
    info!("recieved request to unfavorite article {:?}", slug);

    let article = articles_service.unfavorite_article(user_id, slug).await?;

    Ok(Json(ArticleResponse { article }))
}

pub async fn get_comments(
    Path(slug): Path<String>,
    Extension(comments_service): Extension<DynCommentsService>,
    OptionalAuthentication(user_id): OptionalAuthentication,
) -> AppResult<Json<CommentsResponse>> {
    info!("recieved request to retrieve comments for article {:?}", slug);

    let comments = comments_service.get_comments(user_id, slug).await?;

    Ok(Json(CommentsResponse { comments }))
}

pub async fn add_comment(
    Path(slug): Path<String>,
    Extension(comments_service): Extension<DynCommentsService>,
    RequiredAuthentication(user_id): RequiredAuthentication,
    ValidationExtractor(request): ValidationExtractor<CreateCommentRequest>,
) -> AppResult<Json<CommentResponse>> {
    info!("recieved request to add comment for article {:?}", slug);

    let comment = comments_service
        .add_comment(user_id, slug, request.comment.body.unwrap())
        .await?;

    Ok(Json(CommentResponse { comment }))
}

pub async fn remove_comment(
    Path(comment_id): Path<i64>,
    Extension(comments_service): Extension<DynCommentsService>,
    RequiredAuthentication(user_id): RequiredAuthentication,
) -> AppResult<()> {
    info!("recieved request to remove comment {:?}", comment_id);

    comments_service.remove_comment(user_id, comment_id).await?;

    Ok(())
}

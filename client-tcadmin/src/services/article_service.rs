use domain::{
    articles::{
        models::CreateArticleDto,
        requests::CreateArticleRequest,
        responses::{ArticleResponse, ArticlesResponse},
    },
    comments::responses::CommentsResponse,
};
use lazy_static::lazy_static;

use crate::utilities::{
    errors::{ConduitWebError, ConduitWebResult},
    http::{get, post},
    params::PaginationQueryBuilder,
};

lazy_static! {
    static ref ARTICLES_ENDPOINT: &'static str = "/articles";
}

pub async fn create_article(
    title: String,
    description: String,
    body: String,
    tags: Vec<String>,
) -> ConduitWebResult<ArticleResponse> {
    let article_dto = CreateArticleDto {
        title: Some(title.clone()),
        description: Some(description),
        body: Some(body),
        tag_list: tags,
    };

    let create_article_response = post::<ArticleResponse, CreateArticleRequest>(
        *ARTICLES_ENDPOINT,
        CreateArticleRequest { article: article_dto },
    )
    .await;

    if let Ok(article_response) = create_article_response {
        return Ok(article_response);
    }

    Err(ConduitWebError::ArticleNotCreated)
}

pub async fn get_article(slug: String) -> ConduitWebResult<ArticleResponse> {
    let get_article_response = get::<ArticleResponse>(&format!("{}/{}", *ARTICLES_ENDPOINT, slug)).await;

    if let Ok(article_response) = get_article_response {
        return Ok(article_response);
    }

    Err(ConduitWebError::ArticleNotFound)
}

pub async fn get_articles(
    limit: usize,
    offset: usize,
    author: String,
    tag: String,
    favorited: String,
) -> ConduitWebResult<ArticlesResponse> {
    let url = (*ARTICLES_ENDPOINT).to_string();

    let param_builder = PaginationQueryBuilder::new(url)
        .with_limit(limit)
        .with_offset(offset)
        .with_author(author)
        .with_tag(tag)
        .with_favorited(favorited)
        .build();

    let get_article_response = get::<ArticlesResponse>(&param_builder.to_query_string()).await;

    if let Ok(article_response) = get_article_response {
        return Ok(article_response);
    }

    Err(ConduitWebError::ArticleNotFound)
}

pub async fn get_article_comments(slug: String) -> ConduitWebResult<CommentsResponse> {
    let get_article_comments_response =
        get::<CommentsResponse>(&format!("{}/{}/comments", *ARTICLES_ENDPOINT, slug)).await;

    if let Ok(comments_response) = get_article_comments_response {
        return Ok(comments_response);
    }

    Err(ConduitWebError::CommentsNotLoaded)
}

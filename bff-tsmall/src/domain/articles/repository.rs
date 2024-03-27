use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use serde::Deserialize;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;
// use time::Format;
// use time::format_description;
// use time::macros::format_description;

use crate::application::articles::{ArticleDto, AuthorDto};

pub type DynArticlesRepository = Arc<dyn ArticlesRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait ArticlesRepository {
    async fn create_article(
        &self,
        user_id: i64,
        title: String,
        slug: String,
        description: String,
        body: String,
    ) -> anyhow::Result<UpsertArticleQuery>;

    async fn update_article(
        &self,
        id: i64,
        title: String,
        slug: String,
        description: String,
        body: String,
    ) -> anyhow::Result<UpsertArticleQuery>;

    async fn get_articles(
        &self,
        user_id: Option<i64>,
        tag: Option<String>,
        author: Option<String>,
        favorited: Option<String>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<GetArticleQuery>>;

    async fn get_article_by_slug(&self, user_id: Option<i64>, slug: String) -> anyhow::Result<Option<GetArticleQuery>>;

    async fn delete_article(&self, id: i64) -> anyhow::Result<()>;

    async fn favorite_article(&self, article_id: i64, user_id: i64) -> anyhow::Result<GetArticleQuery>;

    async fn unfavorite_article(&self, article_id: i64, user_id: i64) -> anyhow::Result<GetArticleQuery>;

    async fn get_user_favorites(&self, article_id: i64) -> anyhow::Result<Vec<GetArticleFavoritesQuery>>;
}

#[derive(FromRow, Deserialize)]
pub struct UpsertArticleQuery {
    pub id: i64,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub updated_at: OffsetDateTime,
    pub title: String,
    pub body: String,
    pub description: String,
    pub slug: String,
    pub author_username: String,
    pub author_image: String,
    pub author_bio: String,
}

#[derive(FromRow, Deserialize)]
pub struct GetArticleQuery {
    pub id: i64,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub updated_at: OffsetDateTime,
    pub title: String,
    pub body: String,
    pub description: String,
    pub slug: String,
    pub user_id: i64,
    pub favorites: i64,
    pub favorited: bool,
    pub following_author: bool,
    pub author_username: String,
    pub author_image: String,
    pub author_bio: String,
}

#[derive(FromRow)]
pub struct GetArticleFavoritesQuery {
    pub id: i64,
    pub article_id: i64,
    pub user_id: i64,
}

impl UpsertArticleQuery {
    pub fn into_dto(self, tag_list: Vec<String>) -> ArticleDto {
        ArticleDto {
            id: self.id,
            title: self.title,
            body: self.body,
            tag_list,
            created_at: self.created_at.to_string(),
            updated_at: self.updated_at.to_string(),
            description: self.description,
            slug: self.slug,
            favorited: false,
            favorites_count: 0,
            author: AuthorDto {
                username: self.author_username,
                bio: Some(self.author_bio),
                image: Some(self.author_image),
                following: false,
            },
        }
    }
}

impl GetArticleQuery {
    pub fn into_dto(self, tag_list: Vec<String>) -> ArticleDto {
        ArticleDto {
            id: self.id,
            title: self.title,
            body: self.body,
            tag_list,
            created_at: self.created_at.to_string(),
            updated_at: self.updated_at.to_string(),
            description: self.description,
            slug: self.slug,
            favorited: self.favorited,
            favorites_count: self.favorites,
            author: AuthorDto {
                username: self.author_username,
                bio: Some(self.author_bio),
                image: Some(self.author_image),
                following: self.following_author,
            },
        }
    }
}

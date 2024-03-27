use async_trait::async_trait;
use itertools::Itertools;

use crate::domain::articles::DynArticlesRepository;
use crate::domain::comments::DynCommentsRepository;
use crate::domain::comments::CommentsService;
use crate::infrastructure::errors::{AppError, AppResult};

use super::dto::CommentDto;

pub struct AppCommentsService {
    comments_repository: DynCommentsRepository,
    articles_repository: DynArticlesRepository,
}

impl AppCommentsService {
    pub fn new(comments_repository: DynCommentsRepository, articles_repository: DynArticlesRepository) -> Self {
        Self {
            comments_repository,
            articles_repository,
        }
    }
}

#[async_trait]
impl CommentsService for AppCommentsService {
    async fn get_comments(&self, user_id: Option<i64>, slug: String) -> AppResult<Vec<CommentDto>> {
        // verify the article exists before adding comments
        let article = self.articles_repository.get_article_by_slug(None, slug).await?;

        if let Some(existing_article) = article {
            let comments = self
                .comments_repository
                .get_comments(user_id, existing_article.id)
                .await?
                .into_iter()
                .map_into::<CommentDto>()
                .collect_vec();

            return Ok(comments);
        }

        return Err(AppError::NotFound(String::from("article not found for comments")));
    }

    async fn add_comment(&self, user_id: i64, slug: String, body: String) -> AppResult<CommentDto> {
        // verify the article exists before adding comments
        let article = self.articles_repository.get_article_by_slug(None, slug).await?;

        if let Some(existing_article) = article {
            let comments = self
                .comments_repository
                .create_comment(existing_article.id, user_id, body)
                .await?
                .into();

            return Ok(comments);
        }

        return Err(AppError::NotFound(String::from("article not found for comments")));
    }

    async fn remove_comment(&self, user_id: i64, comment_id: i64) -> AppResult<()> {
        // verify the comment exists before removing
        let comment = self.comments_repository.get_comment(comment_id).await?;

        if let Some(existing_comment) = comment {
            // verify the commenter ID and the request ID match before removing
            if existing_comment.user_id != user_id {
                return Err(AppError::Unauthorized);
            }

            self.comments_repository.delete_comment(comment_id).await?;

            return Ok(());
        }

        return Err(AppError::NotFound(String::from("comment not found")));
    }
}

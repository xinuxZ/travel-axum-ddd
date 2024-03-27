use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::application::comments::CreateCommentCommand;
use crate::application::comments::CommentDto;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCommentRequest {
    #[validate]
    pub comment: CreateCommentCommand,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CommentResponse {
    pub comment: CommentDto,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CommentsResponse {
    pub comments: Vec<CommentDto>,
}

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Default, Validate)]
pub struct CreateCommentCommand {
    #[validate(required)]
    pub body: Option<String>,
}

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug)]
pub struct PageUsersQuery {
    // #[validate]
    pub limit: i64,
    pub offset: i64,
}

#[derive(Clone, Serialize, Deserialize, Debug, Validate, Default)]
pub struct RegisterUserCommand {
    #[validate(required, length(min = 1))]
    pub username: Option<String>,
    #[validate(required, length(min = 1), email(message = "邮箱格式错误"))]
    pub email: Option<String>,
    #[validate(required, length(min = 1))]
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct LoginUserCommand {
    #[validate(required)]
    pub email: Option<String>,
    #[validate(required, length(min = 1))]
    pub password: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UpdateUserCommand {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
}

impl RegisterUserCommand {
    pub fn new_stub() -> Self {
        Self {
            username: Some(String::from("stub username")),
            email: Some(String::from("stub email")),
            password: Some(String::from("stub password")),
        }
    }
}

impl LoginUserCommand {
    pub fn new_stub() -> Self {
        Self {
            email: Some(String::from("stub email")),
            password: Some(String::from("stub password")),
        }
    }
}

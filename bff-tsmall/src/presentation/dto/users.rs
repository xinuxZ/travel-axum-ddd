use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::application::users::LoginUserCommand;
use crate::application::users::PageUsersQuery;
use crate::application::users::RegisterUserCommand;
use crate::application::users::UpdateUserCommand;
use crate::application::users::UserDto;

lazy_static! {
    pub static ref LIMIT: i64 = 20;
    pub static ref OFFSET: i64 = 0;
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct GetUserRequest {
    // #[validate]
    user_id: i64,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PageUsersRequest {
    // #[validate]
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// impl From<PageUsersRequest> for PageUsersQuery {
//     fn from(request: PageUsersRequest) -> Self {
//         Self {
//             limit: request.limit.unwrap_or_else(|| LIMIT.abs()),
//             offset: request.offset.unwrap_or_else(|| OFFSET.abs()),
//         }
//     }
// }

#[derive(Serialize, Deserialize, Debug, Validate, Default)]
pub struct RegisterUserRequest {
    #[validate]
    pub user: RegisterUserCommand,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct LoginUserRequest {
    #[validate]
    pub user: LoginUserCommand,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UpdateUserRequest {
    pub user: UpdateUserCommand,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserAuthenicationResponse {
    pub user: UserDto,
}

#[derive(Serialize, Deserialize, Default)]
pub struct UsersResponse {
    pub users: Vec<UserDto>,

    // #[serde(rename = "usersCount")]
    pub users_count: usize,
}

impl UserAuthenicationResponse {
    pub fn new(
        id: i64,
        username: String,
        email: String,
        // unfortunately, while our implementation returns thes optional fields as empty strings,
        // the realworld demo API enables nullable serializing by default, so we have to wrap these
        // strings as `Option` option values for now
        bio: Option<String>,
        image: Option<String>,
        token: String,
    ) -> Self {
        UserAuthenicationResponse {
            user: UserDto {
                id,
                username,
                email,
                bio,
                image,
                token,
            },
        }
    }
}

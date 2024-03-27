use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;

use crate::application::users::{LoginUserCommand, PageUsersQuery, RegisterUserCommand, UpdateUserCommand, UserDto};
use crate::infrastructure::errors::AppResult;

/// A reference counter for our user service allows us safely pass instances user utils
/// around which themselves depend on the user repostiory, and ultimately, our Posgres connection pool.
pub type DynUsersService = Arc<dyn UsersService + Send + Sync>;

#[automock]
#[async_trait]
pub trait UsersService {
    async fn page_users(&self, query: PageUsersQuery) -> AppResult<Vec<UserDto>>;

    async fn register_user(&self, request: RegisterUserCommand) -> AppResult<UserDto>;

    async fn login_user(&self, request: LoginUserCommand) -> AppResult<UserDto>;

    async fn get_current_user(&self, user_id: i64) -> AppResult<UserDto>;

    async fn updated_user(&self, user_id: i64, request: UpdateUserCommand) -> AppResult<UserDto>;
}

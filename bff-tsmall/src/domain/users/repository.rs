use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;

use crate::domain::users::UserEntity;

/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynUsersRepository = Arc<dyn UsersRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait UsersRepository {
    async fn page_users(&self, limit: i64, offset: i64) -> anyhow::Result<Vec<UserEntity>>;

    async fn search_user_by_email_or_username(&self, email: &str, username: &str)
        -> anyhow::Result<Option<UserEntity>>;

    async fn create_user(&self, email: &str, username: &str, hashed_password: &str) -> anyhow::Result<UserEntity>;

    async fn get_user_by_email(&self, email: &str) -> anyhow::Result<Option<UserEntity>>;

    async fn get_user_by_username(&self, username: &str) -> anyhow::Result<Option<UserEntity>>;

    async fn get_user_by_id(&self, id: i64) -> anyhow::Result<UserEntity>;

    async fn update_user(
        &self,
        id: i64,
        email: String,
        username: String,
        password: String,
        bio: String,
        image: String,
    ) -> anyhow::Result<UserEntity>;
}

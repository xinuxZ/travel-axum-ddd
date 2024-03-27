use std::sync::Arc;

use mockall::automock;

use crate::infrastructure::errors::AppResult;

/// A security service for handling JWT authentication.
pub type DynTokenService = Arc<dyn TokenService + Send + Sync>;

#[automock]
pub trait TokenService {
    fn new_token(&self, user_id: i64, email: &str) -> AppResult<String>;
    fn get_user_id_from_token(&self, token: String) -> AppResult<i64>;
}

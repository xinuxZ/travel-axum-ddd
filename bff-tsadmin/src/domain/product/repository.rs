use async_trait::async_trait;
use std::sync::Arc;

use mockall::automock;

use crate::{domain::product::ProductEntity, domain::product::ProductVo};

/// Similar to above, we want to keep a reference count across threads so we can manage our
/// connection pool.
pub type DynProductRepository = Arc<dyn ProductRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait ProductRepository {
    // async fn create_supplier(&self, supplier_code: &str, supplier_name: &str) ->
    // anyhow::Result<ProductEntity>;

    // async fn update_supplier(&self, id: i64, supplier_name: String) ->
    // anyhow::Result<ProductEntity>;

    // async fn page(&self, limit: i64, offset: i64) -> anyhow::Result<Vec<ProductEntity>>;
    async fn list(&self, r: ProductVo) -> anyhow::Result<Vec<ProductEntity>>;
    async fn get(&self, id: i64) -> anyhow::Result<Option<ProductEntity>>;
}

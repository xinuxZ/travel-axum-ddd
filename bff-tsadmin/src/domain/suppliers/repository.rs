use async_trait::async_trait;
use std::sync::Arc;

use mockall::automock;

use crate::domain::suppliers::SupplierEntity;

/// Similar to above, we want to keep a reference count across threads so we can manage our
/// connection pool.
pub type DynSuppliersRepository = Arc<dyn SuppliersRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait SuppliersRepository {
    // async fn page_suppliers(&self, limit: i64, offset: i64) -> anyhow::Result<Vec<SupplierEntity>>;

    // async fn create_supplier(&self, supplier_code: &str, supplier_name: &str) ->
    // anyhow::Result<SupplierEntity>;

    // async fn get_supplier_by_code(&self, supplier_code: &str) ->
    // anyhow::Result<Option<SupplierEntity>>;

    // async fn get_supplier_by_name(&self, supplier_name: &str) ->
    // anyhow::Result<Option<SupplierEntity>>;

    async fn get(&self, id: i64) -> anyhow::Result<Option<SupplierEntity>>;

    // async fn update_supplier(&self, id: i64, supplier_name: String) ->
    // anyhow::Result<SupplierEntity>;
}

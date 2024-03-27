use async_trait::async_trait;
use std::sync::Arc;

use mockall::automock;

use crate::application::suppliers::SupplierDto;
use crate::infrastructure::app_errors::AppResult;

/// A reference counter for our supplier service allows us safely pass instances supplier utils
/// around which themselves depend on the supplier repostiory, and ultimately, our Posgres
/// connection pool.
#[allow(dead_code)]
pub type DynSuppliersService = Arc<dyn SuppliersService + Send + Sync>;

#[automock]
#[async_trait]
#[allow(dead_code)]
pub trait SuppliersService {
    // async fn page_suppliers(&self, query: PageSuppliersQuery) -> AppResult<Vec<SupplierDto>>;

    async fn get(&self, supplier_id: i64) -> AppResult<SupplierDto>;

    // async fn updated_supplier(&self, supplier_id: i64, request: UpdateSupplierCommand) ->
    // AppResult<SupplierDto>;
}

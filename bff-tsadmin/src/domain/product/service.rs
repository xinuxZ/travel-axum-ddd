use async_trait::async_trait;
use mockall::automock;
use std::sync::Arc;

use crate::application::product::ProductDto;
use crate::infrastructure::app_errors::AppResult;

/// A reference counter for our supplier service allows us safely pass instances supplier utils
/// around which themselves depend on the supplier repostiory, and ultimately, our Posgres
/// connection pool.
#[allow(dead_code)]
pub type DynProductService = Arc<dyn ProductService + Send + Sync>;

#[automock]
#[async_trait]
#[allow(dead_code)]
pub trait ProductService {
    // async fn page_product(&self, query: PageProductQuery) -> AppResult<Vec<ProductDto>>;

    async fn get(&self, supplier_id: i64) -> AppResult<ProductDto>;

    // async fn updated_supplier(&self, supplier_id: i64, request: UpdateProductCommand) ->
    // AppResult<ProductDto>;
}

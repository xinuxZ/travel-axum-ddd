// use async_trait::async_trait;
use tracing::info;

use crate::{domain::suppliers::repository::DynSuppliersRepository, infrastructure::errors::AppResult};

use super::{from_entity_to_dto, SupplierDto};

#[derive(Clone)]
pub struct AppSupplierService {
    repository: DynSuppliersRepository,
}

impl AppSupplierService {
    pub fn new(repository: DynSuppliersRepository) -> Self {
        Self { repository }
    }
}

// #[async_trait]
impl AppSupplierService {
    pub async fn get(&self, id: i64) -> AppResult<SupplierDto> {
        info!("retriveving user {:?}", id);
        let supplier = self.repository.get(id).await?;

        Ok(from_entity_to_dto(supplier))
    }
}

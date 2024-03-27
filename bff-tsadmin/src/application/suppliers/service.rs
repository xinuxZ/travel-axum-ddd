use tracing::info;

use crate::{
    domain::suppliers::repository::DynSuppliersRepository,
    infrastructure::app_errors::{AppError, AppResult},
};

use super::SupplierDto;

#[derive(Clone)]
pub struct SupplierAppService {
    repository: DynSuppliersRepository,
}

impl SupplierAppService {
    pub fn new(repository: DynSuppliersRepository) -> Self {
        Self { repository }
    }
}

impl SupplierAppService {
    pub async fn get(&self, id: i64) -> AppResult<SupplierDto> {
        info!("retriveving supplier {:?}", id);
        let supplier = self.repository.get(id).await?;

        if let Some(existing_supplier) = supplier {
            return Ok(SupplierDto::from(existing_supplier));
        }

        Err(AppError::NotFound(String::from("supplier not found")))
    }
}

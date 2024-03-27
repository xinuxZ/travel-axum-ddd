use async_trait::async_trait;

use crate::domain::{suppliers::repository::SuppliersRepository, SupplierEntity};

use crate::infrastructure::dao::supplier_dto::SupplierDAO;

#[derive(Clone)]
pub struct SupplierRepository {
    dao: SupplierDAO,
}
impl SupplierRepository {
    pub fn new(dao: SupplierDAO) -> Self {
        Self { dao }
    }
}

#[async_trait]
impl SuppliersRepository for SupplierRepository {
    async fn get(&self, id: i64) -> anyhow::Result<SupplierEntity> {
        let supplier_do = self.dao.get(id).await?;

        let supplier = SupplierEntity {
            id: supplier_do.id,
            supplier_type: supplier_do.supplier_type,
            supplier_code: supplier_do.supplier_code,
            supplier_name: supplier_do.supplier_name,
            created_at: supplier_do.created_at,
            updated_at: supplier_do.updated_at,
        };

        return Ok(supplier);
    }
}

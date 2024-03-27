use async_trait::async_trait;

use crate::domain::suppliers::{repository::SuppliersRepository, SupplierEntity};
use crate::infrastructure::mysql::supplier::SupplierDAO;

#[derive(Clone)]
pub struct MySqlSupplierRepository {
    dao: SupplierDAO,
}
impl MySqlSupplierRepository {
    pub fn new(dao: SupplierDAO) -> Self {
        Self { dao }
    }
}

#[async_trait]
impl SuppliersRepository for MySqlSupplierRepository {
    async fn get(&self, id: i64) -> anyhow::Result<Option<SupplierEntity>> {
        let supplier = self.dao.get(id).await?;

        // let supplier_do = supplier.take();

        if let Some(supplier_do) = supplier {
            let supplier = SupplierEntity {
                id: supplier_do.id,
                supplier_type: supplier_do.supplier_type,
                supplier_code: supplier_do.supplier_code,
                supplier_name: supplier_do.supplier_name,
                created_at: supplier_do.created_at,
                updated_at: supplier_do.updated_at,
            };

            return Ok(Some(supplier));
        }

        Ok(None)
    }
}

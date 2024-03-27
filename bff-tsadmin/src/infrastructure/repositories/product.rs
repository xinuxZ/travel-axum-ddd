use async_trait::async_trait;

use crate::domain::product::ProductVo;
use crate::domain::product::{repository::ProductRepository, ProductEntity};
use crate::infrastructure::mysql::model::product::ProductRequest;
use crate::infrastructure::mysql::product::ProductDAO;

#[derive(Clone)]
pub struct MySqlProductRepository {
    dao: ProductDAO,
}
impl MySqlProductRepository {
    pub fn new(dao: ProductDAO) -> Self {
        Self { dao }
    }
}

#[async_trait]
impl ProductRepository for MySqlProductRepository {
    async fn get(&self, id: i64) -> anyhow::Result<Option<ProductEntity>> {
        let product = self.dao.get(id).await?;

        if let Some(product_do) = product {
            let product = product_do.to_entity();
            return Ok(Some(product));
        }

        Ok(None)
    }

    async fn list(&self, r: ProductVo) -> anyhow::Result<Vec<ProductEntity>> {
        let product_do = ProductRequest::from_product_vo(r);
        let product_do_list = self.dao.list(product_do).await?;

        let mut products: Vec<ProductEntity> = Vec::new();

        for product_do in product_do_list {
            let product = product_do.to_entity();

            products.push(product);
        }

        Ok(products)
    }
}

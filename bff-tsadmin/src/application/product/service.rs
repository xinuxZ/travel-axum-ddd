use tracing::info;

use crate::{
    domain::product::repository::DynProductRepository,
    infrastructure::app_errors::{AppError, AppResult},
};

use super::ProductDto;
use super::ProductQuery;

#[derive(Clone)]
pub struct ProductAppService {
    repository: DynProductRepository,
}

impl ProductAppService {
    pub fn new(repository: DynProductRepository) -> Self {
        Self { repository }
    }
}

impl ProductAppService {
    pub async fn get(&self, id: i64) -> AppResult<ProductDto> {
        info!("retriveving product {:?}", id);
        let product = self.repository.get(id).await?;

        if let Some(existing_product) = product {
            return Ok(ProductDto::from(existing_product));
        }

        Err(AppError::NotFound(String::from("product not found")))
    }

    pub async fn list(&self, r: ProductQuery) -> AppResult<Vec<ProductDto>> {
        info!("retriveving product {:?}", &r);

        let product_vo = r.to_domain_vo();
        let products = self.repository.list(product_vo).await?;

        let mut dtos: Vec<ProductDto> = Vec::new();
        for entity in products {
            let dto = entity;

            dtos.push(ProductDto::from(dto));
        }
        if dtos.len() > 0 {
            return Ok(dtos);
        }

        Err(AppError::NotFound(String::from("product not found")))
    }
}

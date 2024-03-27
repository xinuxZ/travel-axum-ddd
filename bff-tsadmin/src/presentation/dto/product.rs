use serde::{Deserialize, Serialize};

use crate::application::product::{ProductDto, ProductQuery};

#[derive(Serialize, Deserialize)]
pub struct ProductResponse {
    pub product: ProductDto,
}

#[derive(Serialize, Deserialize)]
pub struct ProductsResponse {
    pub products: Vec<ProductDto>,
}

#[derive(Debug, Deserialize)]
pub struct ProductRequest {
    pub id: Option<i64>,
    pub tenant_id: Option<i64>,
    pub supplier_id: Option<i64>,
    pub product_title: Option<String>,
    pub product_category_id: Option<i64>,
    pub cooperation_id: Option<i64>,
    pub product_type: Option<i8>,
}

impl ProductRequest {
    pub fn to_cqe(self) -> ProductQuery {
        ProductQuery {
            id: self.id,
            tenant_id: self.tenant_id,
            supplier_id: self.supplier_id,
            product_title: self.product_title,
            product_category_id: self.product_category_id,
            cooperation_id: self.cooperation_id,
            product_type: self.product_type,
        }
    }
}

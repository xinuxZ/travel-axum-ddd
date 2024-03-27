use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
// use sqlx::types::time::OffsetDateTime;
// use validator::Validate;
use crate::domain::product::ProductEntity;
use serde_with::serde_as;
use time::format_description::well_known::Rfc3339;

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct ProductDto {
    pub id: i64,
    pub product_title: String,

    #[serde_as(as = "Rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde_as(as = "Rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl From<ProductEntity> for ProductDto {
    fn from(product: ProductEntity) -> Self {
        ProductDto {
            id: product.id,
            product_title: product.product_title,

            created_at: product.created_at,
            updated_at: product.updated_at,
        }
    }
}

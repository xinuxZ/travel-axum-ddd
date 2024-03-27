use crate::domain::product::{ProductEntity, ProductVo};
use crate::infrastructure::paginate::PaginateRequest;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;
use time::format_description::well_known::Rfc3339;

#[derive(Debug, Default, Deserialize, Serialize, Clone, sqlx::Type)]
#[repr(u8)]
pub enum ProductType {
    #[default]
    Direct,
    Proxy,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProductDO {
    pub id: i64,
    pub tenant_id: i64,
    pub supplier_id: i64,
    pub product_title: String,
    pub product_category_id: i64,
    pub cooperation_id: i64,
    pub product_type: i8,
    pub origin_amount: i64,
    pub actual_amount: i64,
    pub settlement_amount: i64,
    pub commision_amount: i64,
    pub service_type: i8,
    #[serde_as(as = "Rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde_as(as = "Rfc3339")]
    pub updated_at: OffsetDateTime,
    // #[serde_as(as = "Rfc3339")]
    // pub deleted_at: OffsetDateTime,
}

impl ProductDO {
    pub fn to_entity(self) -> ProductEntity {
        ProductEntity {
            id: self.id,
            supplier_id: self.supplier_id,
            tenant_id: self.tenant_id,
            product_category_id: self.product_category_id,
            cooperation_id: self.cooperation_id,
            product_type: self.product_type,
            product_title: self.product_title,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

// #[serde_as]
// #[derive(Debug, Serialize, Deserialize, FromRow)]
#[derive(Default)]
pub struct ProductRequest {
    pub paginate: PaginateRequest,

    pub id: Option<i64>,
    pub tenant_id: Option<i64>,
    pub supplier_id: Option<i64>,
    pub product_title: Option<String>,
    pub product_category_id: Option<i64>,
    pub cooperation_id: Option<i64>,
    pub product_type: Option<i8>,
    pub origin_amount: Option<i64>,
    pub actual_amount: Option<i64>,
    pub settlement_amount: Option<i64>,
    pub commision_amount: Option<i64>,
    pub service_type: Option<i8>,

    // #[serde_as(as = "Rfc3339")]
    pub created_at: Option<OffsetDateTime>,
    // #[serde_as(as = "Rfc3339")]
    pub updated_at: Option<OffsetDateTime>,
    // #[serde_as(as = "Rfc3339")]
    // pub deleted_at: Option<OffsetDateTime>,
}
impl ProductRequest {
    pub fn from_product_vo(pvo: ProductVo) -> Self {
        Self { ..Default::default() }
    }
}

#[derive(Default)]
pub struct PageProductRequest {
    pub paginate: PaginateRequest,

    pub id: Option<i64>,
    pub tenant_id: Option<i64>,
    pub supplier_id: Option<i64>,
    pub product_title: Option<String>,
    pub product_category_id: Option<i64>,
    pub cooperation_id: Option<i64>,
    pub product_type: Option<i8>,
    pub service_type: Option<i8>,
}

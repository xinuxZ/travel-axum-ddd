#![feature(default_free_fn)]

// use serde::{Deserialize, Serialize};
// use validator::Validate;
// use serde_with::serde_as;
// use time::format_description::well_known::Rfc3339;
use sqlx::types::time::OffsetDateTime;

use crate::{domain::product::ProductVo, infrastructure::paginate::PaginateRequest};

// #[serde_as]
#[derive(Debug)]
pub struct PageProductQuery {
    pub id: i64,
    pub product_title: String,

    // #[serde_as(as = "Rfc3339")]
    pub created_at: OffsetDateTime,
    // #[serde_as(as = "Rfc3339")]
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Default)]
pub struct ProductQuery {
    pub id: Option<i64>,
    pub tenant_id: Option<i64>,
    pub supplier_id: Option<i64>,
    pub product_title: Option<String>,
    pub product_category_id: Option<i64>,
    pub cooperation_id: Option<i64>,
    pub product_type: Option<i8>,
    // pub service_type: Option<i8>,
    // #[serde_as(as = "Rfc3339")]
    // pub created_at: Option<OffsetDateTime>,
    // #[serde_as(as = "Rfc3339")]
    // pub updated_at: Option<OffsetDateTime>,
    // #[serde_as(as = "Rfc3339")]
    // pub deleted_at: Option<OffsetDateTime>,
}
impl ProductQuery {
    pub fn to_domain_vo(&self) -> ProductVo {
        ProductVo {
            id: self.id,
            ..Default::default()
        }
    }
}

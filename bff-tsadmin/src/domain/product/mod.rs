// use serde::Serialize;
// use serde_with::serde_as;
use sqlx::types::time::OffsetDateTime;
// use sqlx::FromRow;
// use time::format_description::well_known::Rfc3339;
use crate::infrastructure::paginate::PaginateRequest;

pub mod repository;
pub mod service;

// #[serde_as]
// #[derive(FromRow)]
pub struct ProductEntity {
    pub id: i64,
    pub tenant_id: i64,
    pub supplier_id: i64,
    pub product_title: String,
    pub product_category_id: i64,
    pub cooperation_id: i64,
    pub product_type: i8,
    // pub origin_amount: i64,
    // pub actual_amount: i64,
    // pub settlement_amount: i64,
    // pub commision_amount: i64,
    // pub service_type: i8,
    // #[serde_as(as = "Rfc3339")]
    pub created_at: OffsetDateTime,
    // #[serde_as(as = "Rfc3339")]
    pub updated_at: OffsetDateTime,
    // #[serde_as(as = "Rfc3339")]
    // pub deleted_at: OffsetDateTime,
}

#[derive(Default)]
pub struct ProductVo {
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

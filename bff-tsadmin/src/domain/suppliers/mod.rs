use serde::Serialize;
use serde_with::serde_as;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;
use time::format_description::well_known::Rfc3339;

pub mod repository;
pub mod service;

#[serde_as]
#[derive(Serialize, FromRow)]
pub struct SupplierEntity {
    pub id: i64,

    #[serde_as(as = "Rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde_as(as = "Rfc3339")]
    pub updated_at: OffsetDateTime,
    pub supplier_type: i8,
    pub supplier_name: String,
    pub supplier_code: String,
}

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
// use sqlx::types::time::OffsetDateTime;
// use validator::Validate;
use crate::domain::suppliers::SupplierEntity;
use serde_with::serde_as;
use time::format_description::well_known::Rfc3339;

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct SupplierDto {
    pub id: i64,
    pub supplier_name: String,
    pub supplier_code: String,
    pub supplier_type: i8,

    #[serde_as(as = "Rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde_as(as = "Rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl SupplierDto {
    pub fn from(supplier: SupplierEntity) -> Self {
        Self {
            id: supplier.id,
            supplier_name: supplier.supplier_name,
            supplier_code: supplier.supplier_code,
            supplier_type: supplier.supplier_type,

            created_at: supplier.created_at,
            updated_at: supplier.updated_at,
        }
    }
}

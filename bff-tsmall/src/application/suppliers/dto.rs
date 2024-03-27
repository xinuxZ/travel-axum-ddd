use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
// use validator::Validate;
use crate::{domain::suppliers::SupplierEntity, infrastructure::dao::supplier_dto::SupplierType};

#[derive(Serialize, Deserialize, Debug)]
pub struct SupplierDto {
    pub id: i64,
    pub supplier_type: SupplierType,
    // pub supplier_type: u8,
    pub supplier_name: String,
    pub supplier_code: String,

    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub updated_at: OffsetDateTime,
}

pub fn from_entity_to_dto(supplier: SupplierEntity) -> SupplierDto {
    SupplierDto {
        id: supplier.id,
        supplier_type: supplier.supplier_type,
        supplier_name: supplier.supplier_name,
        supplier_code: supplier.supplier_code,
        created_at: supplier.created_at,
        updated_at: supplier.updated_at,
    }
}

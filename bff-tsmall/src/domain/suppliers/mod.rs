// use chrono::{DateTime, Local};
use serde::Deserialize;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;

use crate::infrastructure::dao::supplier_dto::SupplierType;

pub mod repository;
pub mod service;

#[derive(Deserialize, FromRow)]
pub struct SupplierEntity {
    pub id: i64,
    pub supplier_code: String,
    pub supplier_name: String,
    // pub supplier_type: u8,
    pub supplier_type: SupplierType,

    // pub created_at: DateTime<Local>,
    // pub updated_at: DateTime<Local>,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub updated_at: OffsetDateTime,
}

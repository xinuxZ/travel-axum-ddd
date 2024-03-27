use crate::infrastructure::mysql_connection_pool::MySqlConnectionPool;
use anyhow::Context;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;
// use sqlx::{MySqlPool, Row};
use serde::{Deserialize, Serialize};
use std::default;

#[derive(Debug, Default, Deserialize, Serialize, Clone, sqlx::Type)]
#[repr(u8)]
pub enum SupplierType {
    #[default]
    Direct,
    Proxy,
}

#[derive(Debug, Deserialize, FromRow)]
pub struct SupplierDO {
    pub id: i64,
    pub supplier_code: String,
    pub supplier_name: String,
    // pub supplier_type: u8,
    pub supplier_type: SupplierType,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub updated_at: OffsetDateTime,
}

#[derive(Clone)]
pub struct SupplierDAO {
    pool: MySqlConnectionPool,
}
impl SupplierDAO {
    pub fn new(pool: MySqlConnectionPool) -> Self {
        Self { pool }
    }
}

impl SupplierDAO {
    pub async fn get(&self, id: i64) -> anyhow::Result<SupplierDO> {
        let supplier = sqlx::query_as::<_, SupplierDO>(
            "SELECT id,supplier_name,supplier_code,created_at,updated_at FROM supplier where id = ?",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .context("an unexpected error occured query supplier");

        return supplier;
    }
}

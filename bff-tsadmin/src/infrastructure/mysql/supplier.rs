use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;
use time::format_description::well_known::Rfc3339;

use crate::infrastructure::mysql_connection_pool::MySqlConnectionPool;

#[derive(Debug, Default, Deserialize, Serialize, Clone, sqlx::Type)]
#[repr(u8)]
pub enum SupplierType {
    #[default]
    Direct,
    Proxy,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SupplierDO {
    pub id: i64,
    pub supplier_code: String,
    pub supplier_name: String,

    #[serde_as(as = "Rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde_as(as = "Rfc3339")]
    pub updated_at: OffsetDateTime,
    pub supplier_type: i8,
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
    pub async fn get(&self, id: i64) -> anyhow::Result<Option<SupplierDO>> {
        // let supplier = sqlx::query_as::<_, SupplierDO>(
        //     "SELECT id,supplier_name,supplier_code,supplier_type,created_at,updated_at FROM supplier where id = id",
        // )
        // .bind(id)
        // .fetch_one(&self.pool)
        // .await
        // .context("an unexpected error occured query supplier");

        // 选择适用 query_as 宏，因为适用宏可以在编译阶段校验SQL
        sqlx::query_as!(
            SupplierDO,
            r#"SELECT id,supplier_name,supplier_code,supplier_type,created_at,updated_at FROM supplier where id = ?"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .context("an unexpected error occured query supplier")
    }
}

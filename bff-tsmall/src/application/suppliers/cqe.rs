use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
// use validator::Validate;

#[derive(Serialize, Deserialize, Debug)]
pub struct PageSuppliersQuery {
    pub id: i64,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub supplier_name: String,
    pub supplier_code: String,
}

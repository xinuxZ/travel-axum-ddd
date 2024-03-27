use serde::Serialize;
use sqlx::types::time::OffsetDateTime;
// use validator::Validate;
use serde_with::serde_as;
use time::format_description::well_known::Rfc3339;

#[serde_as]
#[derive(Serialize)]
pub struct PageSuppliersQuery {
    pub id: i64,
    pub supplier_name: String,
    pub supplier_code: String,
    pub supplier_type: i8,

    #[serde_as(as = "Rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde_as(as = "Rfc3339")]
    pub updated_at: OffsetDateTime,
}

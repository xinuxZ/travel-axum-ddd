use serde::{Deserialize, Serialize};

use crate::application::suppliers::SupplierDto;

#[derive(Serialize, Deserialize)]
pub struct SupplierResponse {
    pub supplier: SupplierDto,
}

use serde::{Deserialize, Serialize};

use crate::application::profiles::ProfileDto;

#[derive(Deserialize, Default, Serialize)]
pub struct ProfileResponse {
    pub profile: ProfileDto,
}

use std::collections::HashMap;

use amaterasu_macros::api_type;
use serde::{Deserialize, Serialize};

#[api_type("response/user")]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    #[serde(default)]
    pub table_columns: HashMap<String, HashMap<String, bool>>,
    #[serde(default)]
    pub volume: Option<f32>,
}

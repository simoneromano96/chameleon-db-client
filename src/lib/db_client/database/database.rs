use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// An arangoDb database rappresentation
pub struct Database {
    pub name: String,
    pub id: i64,
    pub path: String,
    pub is_system: bool,
}

/// CRUD here
impl Database {}

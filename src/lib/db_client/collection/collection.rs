use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Collection model
pub struct Collection {
    pub id: String,
    pub name: String,
    pub status: isize,
    pub r#type: isize,
    pub is_system: bool,
    pub globally_unique_id: String,
}
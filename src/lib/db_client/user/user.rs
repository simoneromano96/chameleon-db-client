use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
/// Authentication user model
pub struct User {
pub username: String,
pub password: String,
}

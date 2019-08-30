use serde::Deserialize;

#[derive(Deserialize, Clone)]
/// Access Token model
pub struct AccessToken {
    pub jwt: String,
}


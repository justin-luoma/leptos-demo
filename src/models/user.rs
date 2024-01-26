use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub access_token: String,
    pub uuid: String,
    pub refresh_token: String,
    pub email: String,
}

impl User {
    pub fn new(access_token: String, uuid: String, refresh_token: String, email: String) -> Self {
        Self {
            access_token,
            uuid,
            refresh_token,
            email,
        }
    }
}

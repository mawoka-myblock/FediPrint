use serde::Deserialize;
use serde_derive::Serialize;

#[derive(Deserialize, Serialize)]
pub struct CreateUserInput {
    pub password: String,
    pub email: String,
    pub username: String,
    pub display_name: String,
}

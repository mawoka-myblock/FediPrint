use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserInput {
    pub password: String,
    pub email: String,
    pub username: String,
    pub display_name: String,
}

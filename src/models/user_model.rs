use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct GetUserResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
}
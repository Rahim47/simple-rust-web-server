use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
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

#[derive(Serialize)]
pub struct GetAllUsersResponse {
    pub users: Vec<GetUserResponse>,
}

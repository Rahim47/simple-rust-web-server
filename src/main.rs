mod models;
mod services;

use crate::services::db::UserDb;
use actix_web::{App, Error, HttpResponse, HttpServer, Responder, get, post, web};
use models::user_model::User;
use models::user_model::{CreateUserResponse, GetAllUsersResponse, GetUserResponse};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[get("/greet")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[get("/user/{id}")]
async fn get_user(user_id: web::Path<i32>, db: web::Data<UserDb>) -> Result<impl Responder, Error> {
    let user_id = user_id.into_inner();
    let db = db.lock().unwrap();
    let user = db.get(&user_id);
    match user {
        Some(user_data) => Ok(HttpResponse::Ok().json(GetUserResponse {
            id: user_id,
            name: user_data.name.clone(),
            email: user_data.email.clone(),
        })),
        None => Ok(HttpResponse::NotFound().body("User not found")),
    }
}

#[get("/users")]
async fn get_all_users(db: web::Data<UserDb>) -> impl Responder {
    let db = db.lock().unwrap();
    let users: Vec<GetUserResponse> = db
        .iter()
        .map(|(&id, user)| GetUserResponse {
            id,
            name: user.name.clone(),
            email: user.email.clone(),
        })
        .collect();

    HttpResponse::Ok().json(GetAllUsersResponse { users })
}

#[post("/user")]
async fn create_user(user_data: web::Json<User>, db: web::Data<UserDb>) -> impl Responder {
    let mut db = db.lock().unwrap();
    let new_id = db.keys().max().unwrap_or(&0) + 1;
    let name = user_data.name.clone();
    let email = user_data.email.clone();
    db.insert(new_id, user_data.into_inner());
    HttpResponse::Created().json(CreateUserResponse {
        id: new_id,
        name,
        email,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = 9999;
    println!("Server listening on port {}", port);

    let user_db: UserDb = Arc::new(Mutex::new(HashMap::<i32, User>::new()));

    HttpServer::new(move || {
        let app_data = web::Data::new(user_db.clone());
        App::new()
            .app_data(app_data)
            .service(greet)
            .service(get_user)
            .service(get_all_users)
            .service(create_user)
    })
    .bind(("127.0.0.1", port))?
    .workers(2)
    .run()
    .await
}

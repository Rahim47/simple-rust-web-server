mod services;
mod models;
//
use actix_web::{get,post, web, App, HttpResponse, HttpServer, Responder,Error};
use models::user_model::{CreateUserResponse,GetUserResponse};
use models::user_model::User;
//
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
//
use crate::services::db::UserDb;
//
//
#[get("/greet")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}
//
#[get("/user/{id}")]
async fn get_user(user_id: web::Path<i32>, db: web::Data<UserDb>) -> Result<impl Responder, Error> {
    let user_id = user_id.into_inner();
    let db = db.lock().unwrap();
    let user = db.get(&user_id);
    match user {
        Some(user_data) => Ok(HttpResponse::Ok().json(GetUserResponse{
            id: user_id,
            name: user_data.name.clone(),
            email: user_data.email.clone(),
        })),
        None => Ok(HttpResponse::NotFound().body("User not found")),
    }
}
//
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
// git remote add origin https://github.com/Rahim47/simple-rust-web-server.git
//
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let port: u16 = 8080; 
    println!("Server listening on port {}", port);

    let user_db: UserDb = Arc::new(Mutex::new(HashMap::<i32, User>::new()));

    HttpServer::new(move || {
        let app_data = web::Data::new(user_db.clone());
        App::new().app_data(app_data).service(greet)
        .service(get_user)
        .service(create_user)
    })
        .bind(("127.0.0.1", port))?
        .workers(2)
        .run()
        .await
}
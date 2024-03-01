#[macro_use] extern crate rocket;

mod auth;
mod schema;
mod models;
mod repositories;

use diesel::prelude::*;
use auth::BasicAuth;
use rocket::serde::json::{Value, json, Json};
use rocket::response::status;
use rocket_sync_db_pools::database;
use schema::rustaceans;
use models::Rustacean;
use crate::models::NewRustacean;
use crate::repositories::RustaceanRepository;

#[database("sqlite")]
struct DbConn(SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Value {
    db.run(|c| {
        let rustaceans = RustaceanRepository::get(c, 100)
            .expect("Database error");
        json!(rustaceans)
    }).await
}

#[get("/rustaceans/<id>")]
async fn view_rustacean(id: i32, _auth: BasicAuth, db: DbConn) -> Value {
    db.run(move |c| {
        let rustacean = RustaceanRepository::find(c, id)
            .expect("Failed To Search");
        json!(rustacean)
    }).await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(_auth: BasicAuth, db: DbConn, new_rustacean: Json<NewRustacean>) -> Value {
    db.run(|c| {
        let result = RustaceanRepository::create(c, new_rustacean.into_inner())
            .expect("Failed To Add");
        json!(result)
    }).await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(id: i32, _auth: BasicAuth, db: DbConn, rustacean: Json<NewRustacean>) -> Value {
    db.run(move |c| {
        let result = RustaceanRepository::update(c, id ,rustacean.into_inner())
            .expect("Failed To Update");
        json!(result)
    }).await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(id: i32, _auth: BasicAuth, db: DbConn) -> status::NoContent {
    db.run(move |c| {
        RustaceanRepository::delete(c, id)
            .expect("Failed To Delete");
        status::NoContent
    }).await
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not found")
}

#[catch(422)]
fn not_processible() -> Value {
    json!("Some Entry Field(s) Are Missing")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![
            get_rustaceans,
            view_rustacean,
            create_rustacean,
            update_rustacean,
            delete_rustacean
        ])
        .register("/", catchers![
            not_found,
            not_processible
        ])
        .attach(DbConn::fairing())
        .launch()
        .await;
}

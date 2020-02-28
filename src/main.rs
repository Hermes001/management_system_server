#[macro_use]
extern crate diesel;

use std::env;
use actix_web::{get, middleware, post, Responder, web, App, Error, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;
use crate::models::User;

mod actions;
mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let pool = establish_connection();

    HttpServer::new(move || {
        App::new().data(pool.clone()).service(add_user)
    }).bind("127.0.0.1:7878")?.run().await
}

#[post("/user")]
async fn add_user(pool: web::Data<DbPool>, form: web::Json<models::User>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let user = web::block(move || actions::insert_new_user(&form.user_name, &form.password, &conn)).await.map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish();
    })?;
    Ok(HttpResponse::Ok().json(user))
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

fn establish_connection() -> DbPool {
    dotenv::dotenv().ok();

    let database_url = env::var("PG_DATABASE_URL").expect("PG_DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
}
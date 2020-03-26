#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;

mod actions;
mod models;
mod resp;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let pool = establish_connection();
    let bind = "127.0.0.1:7878";

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(resp::login)
            .service(resp::add_user)
            .service(resp::get_user)
    })
    .bind(&bind)?
    .run()
    .await
}

fn establish_connection() -> DbPool {
    dotenv::dotenv().ok();

    let database_url = env::var("PG_DATABASE_URL").expect("PG_DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

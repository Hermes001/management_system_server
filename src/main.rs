#[macro_use]
extern crate diesel;

<<<<<<< HEAD
// use crate::models::User;
use actix_web::{post, web, App, Error, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;
// use uuid::Uuid;
=======
use std::env;
use actix_web::{get, middleware, post, Responder, web, App, Error, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;
use crate::models::User;
>>>>>>> b6f927e0b194c5fd0f473b07e8a415bd797bdd75

mod actions;
mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let pool = establish_connection();

<<<<<<< HEAD
    HttpServer::new(move || App::new().data(pool.clone()).service(add_user))
        .bind("127.0.0.1:7878")?
        .run()
        .await
}

#[post("/user")]
async fn add_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::User>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let user = web::block(move || actions::insert_new_user(&form.user_name, &form.password, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish();
        })?;
    Ok(HttpResponse::Ok().json(user))
}

=======
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

>>>>>>> b6f927e0b194c5fd0f473b07e8a415bd797bdd75
fn establish_connection() -> DbPool {
    dotenv::dotenv().ok();

    let database_url = env::var("PG_DATABASE_URL").expect("PG_DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
<<<<<<< HEAD
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
=======
    r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
}
>>>>>>> b6f927e0b194c5fd0f473b07e8a415bd797bdd75

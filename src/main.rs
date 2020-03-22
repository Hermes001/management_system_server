#[macro_use]
extern crate diesel;

// use crate::models::User;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;
use uuid::Uuid;

mod actions;
mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let pool = establish_connection();
    let bind = "127.0.0.1:7878";

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(add_user)
            .service(get_user)
    })
    .bind(&bind)?
    .run()
    .await
    // HttpServer::new(move || App::new().service(web::resource("/").to(|| HttpResponse::Ok())))
    //     .bind("127.0.0.1:7878")?
    //     .run()
    //     .await
}

#[get("/user/{user_id}")]
async fn get_user(
    pool: web::Data<DbPool>,
    user_uid: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let user_id = user_uid.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || actions::find_user_by_id(user_id, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(format!("No user found with uid"));
        Ok(res)
    }
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

fn establish_connection() -> DbPool {
    dotenv::dotenv().ok();

    let database_url = env::var("PG_DATABASE_URL").expect("PG_DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

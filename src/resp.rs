use crate::actions;
use actix_web::{get, post, web, Error, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::collections::HashMap;

#[post("/user/login")]
pub async fn login(
    pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
    form: web::Form<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let form = form.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    let user =
        web::block(move || actions::check_user(&conn, &form["user_name"], &form["password"]))
            .await
            .map_err(|e| {
                eprint!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

    if let Some(_) = user {
        Ok(HttpResponse::Ok().json("登录成功"))
    } else {
        let res = HttpResponse::NotFound().body(format!("账号或密码错误"));
        Ok(res)
    }
}

#[get("/user/query_user")]
pub async fn get_user(
    pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
    form: web::Form<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let form = form.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || actions::find_user_by_id(&conn, &form["id"]))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(format!("找不到用户"));
        Ok(res)
    }
}

#[post("/user/add")]
pub async fn add_user(
    pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
    form: web::Form<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let form = form.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    let _ = web::block(move || actions::add_user(&conn, &form["user_name"], &form["password"]))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish();
        })?;
    Ok(HttpResponse::Ok().json("添加成功"))
}

#[post("/stock/query_all")]
pub async fn get_stock(
    pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let stock = web::block(move || actions::query_stock(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish();
        })?;
    if let Some(stock) = stock {
        Ok(HttpResponse::Ok().json(stock))
    } else {
        let res = HttpResponse::NotFound().body(format!("找不到用户"));
        Ok(res)
    }
}

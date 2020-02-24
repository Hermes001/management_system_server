use actix_web::{Responder, HttpResponse, HttpServer, App, web};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/index", web::get().to(index))
    }).bind("127.0.0.1:7878")?.run().await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}
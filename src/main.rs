use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let app = || App::new().service(hello);

    HttpServer::new(app).bind(("127.0.0.1", 5000))?.run().await
}

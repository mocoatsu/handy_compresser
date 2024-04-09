use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn compress_file() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/compress", web::get().to(compress_file)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}


use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ZipStyle {
    zip: String,
    style: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

/*
  @param style - list of clothing styles 
**/
#[get("/api/zip")]
async fn clothing_style(info: web::Query<ZipStyle>) -> impl Responder {
    println!("Country Zip: {}", info.zip);
    return HttpResponse::Ok().body(format!("The style is: {}", info.style))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(clothing_style)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
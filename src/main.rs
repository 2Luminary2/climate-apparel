use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;



#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

/*
  @param style - list of clothing styles 
**/
#[get("/{zip}/{style}")]
async fn clothing_style(path: web::Path<String>) -> impl Responder {
    let style = path.into_inner();
    HttpResponse::Ok().body(format!("The style is: {}", style))
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
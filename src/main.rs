use std::env;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct ZipStyle {
    zip: String,
    style: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct GeoLocation {
    zip: String,
    name: String,
    lat: f32,
    lon: f32,
    country: String,
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
    let key = "OPENWEATHER_API_KEY";
    println!("Country Zip: {}", info.zip);
    if let Ok(val) = env::var(key) {
        let response = reqwest::get(format!("http://api.openweathermap.org//geo/1.0/zip?zip={}&appid={}", info.zip, val)).await;
        let body = match response {
            Ok(body) => body,
            Err(_) => return HttpResponse::InternalServerError().body("API request failed")
        };
        let body: GeoLocation = serde_json::from_str(&body.text().await.unwrap()).unwrap();
        println!("Latidude: {}, Longitude: {}", body.lat, body.lon);
        let response = reqwest::get(format!("http://api.openweathermap.org/data/2.5/onecall?lat={}&lon={}&appid={}", body.lat, body.lon, val)).await;
        let body = match response {
            Ok(body) => body,
            Err(_) => return HttpResponse::InternalServerError().body("API request failed")
        };
        println!("{}", val);
        println!("Response: {:?}", body.text().await.unwrap());
        /*if let Ok(body) = response {
            let body: GeoLocation = serde_json::from_str(&body.text().await.unwrap()).unwrap();
            
        } else {
            HttpResponse::InternalServerError().body("API request failed");
            panic!("API request failed {:?}", response);
        }*/
    } else {
        return HttpResponse::InternalServerError().body("API Key not found");
    }
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
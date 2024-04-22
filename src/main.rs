use std::env;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use image_search::Arguments;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map};

#[derive(Debug, Deserialize)]
struct ZipStyle {
    zip: String,
    style: String,
    men: Option<bool>
}

#[derive(Debug, Deserialize, Serialize)]
struct GeoLocation {
    zip: String,
    name: String,
    lat: f32,
    lon: f32,
    country: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Weather {
    temp: f32,
    feels_like: f32,
    humidity: f32,
    weather: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ZipReturn {
    style: String,
    weather: Weather,
    location: GeoLocation,
    urls: Vec<String>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

/*
  @param style - list of clothing styles 
**/


fn get_multiple<T>(object: &serde_json::Value, json_keys: Vec<&str>) -> Option<T> 
where
    T: serde::de::DeserializeOwned,
{
    let mut current = Map::new();
    for key in json_keys {
        let obj = object.get(key).unwrap_or_else(|| &json!(null)).clone();
        current.insert(key.to_string(), obj);
    }
    
    let value = match serde_json::from_value::<T>(json!(current)) {
        Ok(value) => value,
        Err(_) => return None
    };
    return Some(value);

}


#[get("/api/zip")]
async fn clothing_style(info: web::Query<ZipStyle>) -> impl Responder {
    let key = "OPENWEATHER_API_KEY";
    println!("Country Zip: {}", info.zip);
    if let Ok(val) = env::var(key) {
        let response = reqwest::get(format!("http://api.openweathermap.org//geo/1.0/zip?zip={}&appid={}", info.zip, val)).await;
        let body = match response {
            Ok(body) => body,
            Err(_) => return HttpResponse::InternalServerError().body("API request failed - api call failed")
        };
        let geo_body: GeoLocation = serde_json::from_str(&body.text().await.unwrap()).unwrap();
        println!("Latidude: {}, Longitude: {}", geo_body.lat, geo_body.lon);
        let response = reqwest::get(format!("http://api.openweathermap.org/data/3.0/onecall?lat={}&lon={}&appid={}", geo_body.lat, geo_body.lon, val)).await;
        let body = match response {
            Ok(body) => body,
            Err(_) => return HttpResponse::InternalServerError().body("API request failed - api call failed")
        };
        let body = match serde_json::from_str::<serde_json::Value>(&body.text().await.unwrap()) {
            Ok(body) => body,
            Err(_) => return HttpResponse::InternalServerError().body("API request failed - JSON parsing failed")
        };
        let current_weather = match body.get("current"){
            Some(current) => current,
            None => return HttpResponse::InternalServerError().body("API request failed - current weather not found")
        };
        println!("Current Weather: {:?}", current_weather);
        let weather_values = get_multiple(current_weather, vec!["temp", "feels_like", "humidity", "weather"]);
        let weather: Weather = match weather_values {
            Some(weather) => weather,
            None => return HttpResponse::InternalServerError().body("API request failed - missing weather values in object")
        };
        let mut humidity_keyword: &str = "";
        weather.weather.iter().for_each(|value| {
            if let Some(weather) = value.get("description") {
                if let Some(description) = weather.as_str() {
                    if description.contains("rain") {
                        humidity_keyword = "Rainy";
                    }
                    if description.contains("Snow") {
                        humidity_keyword = "Snowing";
                    }
                }
            }   
        });
        let feels_like_celsius = weather.feels_like - 273.15;
        let temp_key_word = if feels_like_celsius < 0.0 {
            "Freezing"
        } else if weather.temp < 15.0 {
            "Cold"
        } else if weather.temp < 25.0 {
            "Warm"
        } else {
            "Hot"
        };
        
        let full_search_key = match info.men {
            Some(true) => format!("{} clothing for {} {} weather mens", info.style, temp_key_word, humidity_keyword),
            Some(false) => format!("{} clothing for {} {} weather", info.style, temp_key_word, humidity_keyword),
            None => format!("{} clothing for {} {} weather", info.style, temp_key_word, humidity_keyword)
        }; 
        let args = Arguments::new(&full_search_key, 5);
        let urls = image_search::urls(args).await;
        if let Ok(urls) = urls {
            return HttpResponse::Ok().json(json!(ZipReturn {
                style: info.style.clone(),
                weather: weather,
                location: geo_body,
                urls: urls
            }));
        } else {
            println!("{:?}", urls);
            return HttpResponse::InternalServerError().body("API request failed - image search failed");
        }


        
    } else {
        return HttpResponse::InternalServerError().body("API Key not found");
    }
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
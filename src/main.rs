use actix_web::{web::{self, Json}, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde_json;

use battlesnake_rust::*;


async fn index(req: HttpRequest) -> impl Responder {
    // println!("Request:\n{:?}", req);

    let response = serde_json::to_string(&RootResponse::default()).unwrap();
    HttpResponse::Ok()
    .content_type("application/json")
    .body(response)

}

async fn start_handler(req: HttpRequest, body: Json<RequestBody>) -> impl Responder {
    // println!("Request:\n{:?}", req);
    println!("Start Body:\n{:?}", body);
    HttpResponse::Ok()
}

async fn move_handler(req: HttpRequest, body: Json<RequestBody>) -> impl Responder {
    // println!("Request:\n{:?}", req);
    println!("Move Body:\n{:?}", body);
    let temp = &body.board.food;
    println!("Food: {:?}", temp);
    let response = serde_json::to_string(&MoveResponse::new(body.get_response(),"Get out of my way!")).unwrap();
    HttpResponse::Ok()
    .content_type("application/json")
    .body(response)
}

async fn end_handler(req: HttpRequest, body: Json<RequestBody>) -> impl Responder {
    // println!("Request:\n{:?}", req);
    println!("Body:\n{:?}", body);
    HttpResponse::Ok()
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    let ip = &std::env::var("IP")
        .unwrap_or_else(|_| "127.0.0.1".to_string());

    println!("Attempting to host at {}:{}", ip, port);
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/start", web::get().to(start_handler))
            .route("/move", web::post().to(move_handler))
            .route("/end", web::post().to(end_handler))
    })
    .bind((ip as &str, port))
    .expect(&format!("Can not bind to port {}",port))
    .run()
    .await
}

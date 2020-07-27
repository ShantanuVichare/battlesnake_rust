use actix_web::{web::{self, Json, Data}, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde_json;

use battlesnake_rust::*;


async fn index(req: HttpRequest) -> impl Responder {
    // println!("Request:\n{:?}", req);

    let response = serde_json::to_string(&RootResponse::default()).unwrap();
    HttpResponse::Ok()
    .content_type("application/json")
    .body(response)
}

async fn start_handler(req: HttpRequest, body: Json<RequestBody>, data: Data<AppStateWrapper>) -> impl Responder {
    // println!("Request:\n{:?}", req);
    // println!("Start Body:\n{:?}", body);

    data.initialise(&body);
    HttpResponse::Ok()
}

async fn move_handler(req: HttpRequest, body: Json<RequestBody>, data: Data<AppStateWrapper>) -> impl Responder {
    // println!("Request:\n{:?}", req);
    // println!("Move Body:\n{:?}", body);
    
    data.update(&body);
    let response = serde_json::to_string(&MoveResponse::new(data.get_response(),"Get out of my way!")).unwrap();
    HttpResponse::Ok()
    .content_type("application/json")
    .body(response)
}

async fn end_handler(req: HttpRequest, body: Json<RequestBody>, data: Data<AppStateWrapper>) -> impl Responder {
    // println!("Request:\n{:?}", req);
    println!("End Body:\n{:?}", body);

    data.end_game(&body);
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

    let mut data = Data::new(AppStateWrapper::new());
    
    println!("Attempting to host at {}:{}", ip, port);
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
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

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
// use serde_derive::Deserialize;
use serde::{Serialize, Deserialize};

// #[derive(Debug, Deserialize)]
// struct Info {
//     username: String,
// }

async fn index(req: HttpRequest) -> impl Responder {
    println!("Request:\n{:?}", req);
    // println!("Body:\n{:?}", body);
    HttpResponse::Ok().body("At the root")
}

async fn start_handler(req: HttpRequest) -> impl Responder {
    println!("Request:\n{:?}", req);
    HttpResponse::Ok().body("Hello!")
}

async fn move_handler(req: HttpRequest) -> impl Responder {
    println!("Request:\n{:?}", req);
    HttpResponse::Ok().body("Move!")
}

async fn end_handler(req: HttpRequest) -> impl Responder {
    println!("Request:\n{:?}", req);
    HttpResponse::Ok().body("Bye!")
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

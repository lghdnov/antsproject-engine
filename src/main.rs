mod backend;
mod game;

use std::env;

use actix_web::{http::uri::Port, post, web, App, HttpServer, Responder};
use deno_core::parking_lot::Mutex;
use crate::backend::pool::{GamePool, GameTask};

struct AppState{
    game_pool: Mutex<GamePool>
}

#[post("/ping")]
async fn ping() -> impl Responder{
    format!("Pong!")
}
#[actix_web::main]
async fn main() -> Result<(), std::io::Error>{
    let mut port = 8081;

    match env::var("PORT") {
        Ok(p) => {port = p.parse::<u16>().unwrap();}
        Err(_) => {}
    }

    println!("Starting api on port {port}");

    let app_data = web::Data::new(AppState{
        game_pool: Mutex::new(GamePool::new(3))
    });

    HttpServer::new(move || App::new()
        .app_data(app_data.clone())
        .service(ping)
        .configure(backend::handler::configure))
        .bind(("127.0.0.1", port))?
        .workers(2)
        .run()
        .await
}

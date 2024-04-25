use actix_web::{App, HttpServer, post, Responder};

#[post("/ping")]
async fn ping() -> impl Responder{
    format!("Pong!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8081;

    println!("Starting api on port {}", port);

    HttpServer::new(|| App::new().service(ping))
        .bind(("127.0.0.1", port))?
        .workers(2)
        .run()
        .await
}

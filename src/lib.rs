use actix_web::{
    dev::Server,
    get, post,
    web::{self},
    App, HttpResponse, HttpServer,
};
use serde::{Deserialize, Serialize};
use std::net::TcpListener;
#[get("/health_check")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Serialize, Deserialize)]
struct FormData {
    name: String,
    email: String,
}
#[post("/subscriptions")]
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check).service(subscribe))
        .listen(listener)?
        .run();
    Ok(server)
}

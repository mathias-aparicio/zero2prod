use actix_web::{
    post,
    web::{self},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
struct FormData {
    name: String,
    email: String,
}
#[post("/subscriptions")]
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

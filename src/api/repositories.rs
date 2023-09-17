use actix_web::{get, web, HttpResponse};

#[get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("repositories")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/repo").service(index));
}

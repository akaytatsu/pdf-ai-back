use actix_web::{web, App, HttpServer};
use std::io::Result;

mod api;
mod models;
mod repository;

#[actix_web::main]
async fn main() -> Result<()> {
    let pool = repository::database::connect_to_database();
    let app_data = web::Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(api::users::config)
            .configure(api::repositories::config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

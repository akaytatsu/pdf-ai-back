use actix_web::{web, App, HttpServer};
use api::broadcast::Broadcaster;
use std::{io::Result, sync::Arc};

mod api;
mod models;
mod repository;

#[actix_web::main]
async fn main() -> Result<()> {
    let broadcaster = Broadcaster::create();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(Arc::clone(&broadcaster)))
            .configure(api::document::routers)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

use actix_files::NamedFile;
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{
    get, post,
    web::{self, scope},
    HttpResponse, Responder,
};
use actix_web_lab::extract::Path;

use crate::repository::{
    data::{get_conversations, get_file, list_folders, save_file},
    utils::get_file_path,
};
use actix_web::Result;

use super::broadcast::Broadcaster;

#[get("/")]
async fn documents() -> HttpResponse {
    let folders = list_folders();

    match folders {
        Ok(folders) => HttpResponse::Ok().json(folders),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[post("/")]
async fn create_document(MultipartForm(form): MultipartForm<UploadForm>) -> HttpResponse {
    if form.files.is_empty() {
        return HttpResponse::BadRequest().finish();
    }

    let mut form = form;
    let temp_file = form.files.remove(0);
    save_file(temp_file).await;

    HttpResponse::Ok().finish()
}

#[get("/{id}")]
async fn get_document(id: web::Path<String>) -> HttpResponse {
    let resp = get_file(id.into_inner());

    match resp {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}/file")]
async fn get_document_file(id: web::Path<String>) -> Result<NamedFile> {
    let path = get_file_path(&id.into_inner(), "pdf");

    Ok(NamedFile::open(path)?)
}

#[get("/{id}/conversation")]
async fn get_document_conversation(id: web::Path<String>) -> impl Responder {
    let _resp = get_conversations(id.into_inner());

    println!("resp33333: {:?}", _resp);

    HttpResponse::Ok().json(_resp)
}

#[get("/{id}/sse")]
async fn event_stream(broadcaster: web::Data<Broadcaster>) -> impl Responder {
    broadcaster.new_client().await
}

#[post("/{id}/{msg}")]
async fn broadcast_msg(
    broadcaster: web::Data<Broadcaster>,
    Path((_id, msg)): Path<(String, String)>,
) -> impl Responder {
    broadcaster.broadcast(&msg).await;
    HttpResponse::Ok().body("msg sent")
}

pub fn routers(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope("/document")
            .service(documents)
            .service(create_document)
            .service(get_document_file)
            .service(event_stream)
            .service(get_document_conversation)
            .service(broadcast_msg)
            .service(get_document),
    );
}

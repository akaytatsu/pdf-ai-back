use actix_web::{get, post, web, HttpResponse};
use serde::Deserialize;
use validator::Validate;

use crate::api::middlewares::AuthorizationService;
use crate::models::users::User;
use crate::repository;
use repository::database::user::{IUserOperations, UserOperations};
use repository::database::DBPool;

#[derive(Deserialize)]
pub struct IndexQuery {
    username: Option<String>,
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[post("/login")]
async fn login(db: web::Data<DBPool>, user: web::Json<LoginRequest>) -> HttpResponse {
    let login_data = user.into_inner();

    let user_operations = UserOperations {
        pool: db.get_ref().clone(),
    };

    match user_operations.login(login_data.email, login_data.password) {
        Ok(login_response) => HttpResponse::Ok().json(login_response),
        Err(response) => HttpResponse::Unauthorized().json(response),
    }
}

#[get("/")]
pub async fn index(info: web::Query<IndexQuery>) -> HttpResponse {
    let user_operations: UserOperations = UserOperations {
        pool: repository::database::connect_to_database(),
    };

    if let Some(username) = &info.username {
        println!("username: {}", username);
    }

    let users = user_operations.get_users();

    HttpResponse::Ok().body(serde_json::to_string(&users).unwrap())
}

#[post("/create")]
pub async fn create(db: web::Data<DBPool>, json: web::Json<User>) -> HttpResponse {
    match json.validate() {
        Ok(_) => (),
        Err(e) => return HttpResponse::BadRequest().json(e),
    }
    let user_operations = UserOperations {
        pool: db.get_ref().clone(),
    };

    let user = user_operations.create(json.into_inner());

    HttpResponse::Ok().body(serde_json::to_string(&user).unwrap())
}

#[get("/securite")]
pub async fn securite(_: AuthorizationService) -> HttpResponse {
    HttpResponse::Ok().body("Securite")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(login)
            .service(index)
            .service(create)
            .service(securite),
    );
}

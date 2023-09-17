use crate::models::users::Claims;
use actix_web::{error::ErrorUnauthorized, Error, FromRequest};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

pub struct AuthorizationService;

impl FromRequest for AuthorizationService {
    type Error = Error;
    type Future = Ready<Result<AuthorizationService, Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let auth = req.headers().get("Authorization");

        match auth {
            Some(_) => {
                let split: Vec<&str> = auth.unwrap().to_str().unwrap().split("Bearer").collect();
                let token = split[1].trim();
                let key = "secret".as_bytes();
                match decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(key),
                    &Validation::new(Algorithm::HS256),
                ) {
                    Ok(_token) => ok(AuthorizationService),
                    Err(_) => err(ErrorUnauthorized("invalid token")),
                }
            }
            None => err(ErrorUnauthorized("invalid token")),
        }
    }
}

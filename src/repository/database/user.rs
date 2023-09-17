use crate::models::schema::users::dsl::*;

use crate::models::users::Claims;
use crate::{
    models::{
        response::{LoginResponse, Response},
        users::User,
    },
    repository::database::DBPool,
};
use chrono::{Duration, Utc};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use diesel::prelude::*;
use jsonwebtoken::{encode, EncodingKey, Header};

pub trait IUserOperations {
    // fn new(pool: DBPool) -> Self;
    fn get_by_email(&self, user_mail: String) -> Option<User>;
    fn login(&self, login_email: String, login_password: String)
        -> Result<LoginResponse, Response>;
    fn get_users(&self) -> Vec<User>;
    fn create(&self, user_data: User) -> Response;
}

pub struct UserOperations {
    pub pool: DBPool,
}

impl IUserOperations for UserOperations {
    // fn new(pool: DBPool) -> Self {
    //     UserOperations { pool }
    // }

    fn get_by_email(&self, user_mail: String) -> Option<User> {
        let conn = &mut self.pool.get().unwrap();

        users
            .filter(email.eq(user_mail))
            .first::<User>(conn)
            .optional()
            .unwrap()
    }

    fn login(
        &self,
        login_email: String,
        login_password: String,
    ) -> Result<LoginResponse, Response> {
        match self.get_by_email(login_email) {
            None => Err(Response {
                message: "Invalid credentials".to_string(),
                status: 401,
            }),
            Some(u) => {
                let mut sha = Sha256::new();
                sha.input_str(&login_password);

                if u.password == sha.result_str() {
                    let my_claims = Claims {
                        sub: u.email,
                        exp: (Utc::now() + Duration::days(365)).timestamp() as usize,
                    };

                    let token = encode(
                        &Header::default(),
                        &my_claims,
                        &EncodingKey::from_secret("secret".as_bytes()),
                    )
                    .unwrap();

                    Ok(LoginResponse {
                        message: "Login successful".to_string(),
                        status: 200,
                        token,
                    })
                } else {
                    Err(Response {
                        message: "Invalid credentials".to_string(),
                        status: 401,
                    })
                }
            }
        }
    }

    fn get_users(&self) -> Vec<User> {
        let conn = &mut self.pool.get().unwrap();

        users.load::<User>(conn).unwrap()
    }

    fn create(&self, user_data: User) -> Response {
        let conn = &mut self.pool.get().unwrap();

        let mut sha = Sha256::new();
        sha.input_str(&user_data.password);

        let new_user = User {
            password: sha.result_str(),
            ..user_data
        };

        match diesel::insert_into(users).values(&new_user).execute(conn) {
            Ok(_) => Response {
                message: "User created".to_string(),
                status: 200,
            },
            Err(_) => Response {
                message: "Error creating user".to_string(),
                status: 500,
            },
        }
    }
}

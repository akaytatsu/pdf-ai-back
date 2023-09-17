use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, Validate)]
#[diesel(table_name = crate::models::schema::users)]
pub struct User {
    #[serde(default)]
    pub id: i32,

    #[validate(length(min = 3))]
    pub name: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

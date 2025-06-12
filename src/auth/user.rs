

use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use crate::schema::users;

#[derive(Serialize, Deserialize)]
pub struct Claims{
    pub sub: i32,
    pub exp: i64
}

#[derive(Queryable, Serialize, Debug)]
pub struct User {
    pub user_id: i32,
    pub name: String,
    pub password: String,
    pub email: String
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub password: String,
    pub email: String
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String
}

#[derive(Serialize, Debug)]
pub struct PublicUserDetails {
    pub name: String,
    pub email: String
}
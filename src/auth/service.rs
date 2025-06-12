
use std::io;
use std::env;
use chrono::Duration;
use dotenv::dotenv;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::result::Error;
use diesel::PgConnection;
use r2d2::PooledConnection;
use crate::schema;
use crate::schema::users::dsl::*;
use chrono::Utc;

use crate::auth::user::{NewUser, LoginUser, User};
use crate::auth::user::Claims;
use crate::db::DbPool;
use jsonwebtoken::{Validation, DecodingKey, EncodingKey, decode, encode, TokenData, Header};


use bcrypt::{hash, verify, DEFAULT_COST};


pub struct AuthService {
    pub pool: DbPool,
}

impl AuthService {
    fn get_conn(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Error> {
        self.pool.get().map_err(|e| Error::DatabaseError(diesel::result::DatabaseErrorKind::UnableToSendCommand, Box::new(e.to_string())))
    }

    pub fn create_user(&self, mut new_user: NewUser) -> Result<(), Error> {
        let mut conn = self.get_conn()?;

        let hashed = hash(&new_user.password, DEFAULT_COST).map_err(|e| Error::SerializationError(Box::new(io::Error::new(io::ErrorKind::Other, e))))?;
        new_user.password = hashed;

        diesel::insert_into(schema::users::table)
            .values(&new_user)
            .execute(&mut conn)?;

        Ok(())
    }

    pub fn login(&self, login_data: LoginUser) -> Option<User> {
        let mut conn = match self.get_conn() {
            Ok(conn) => conn,
            Err(_) => return None,
        };

        let user: User = users
            .filter(email.eq(&login_data.email))
            .first(&mut conn)
            .ok()?;

        if verify(&login_data.password, &user.password).ok()? {
            Some(user)
        }else{
            None
        }
    }
}


pub fn generate_jwt(other_user_id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    dotenv().ok();
        
    let secret = env::var("JWT_SECRET").expect("No JWT env variable defined");

    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(3600))
        .expect("Timestamp invalid.")
        .timestamp();

    let claims = Claims {
        sub: other_user_id.to_owned(),
        exp: expiration,
    };

    let mut header = Header::default();
    header.typ = Some("JWT".to_string());

    encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

pub fn decode_token(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("No JWT env variable defined");

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
}

pub fn is_valid(claim: &Claims) -> bool {
    claim.exp > Utc::now().timestamp()
}
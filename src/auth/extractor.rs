use actix_web::{FromRequest, HttpRequest, Error as ActixError, dev::Payload, http::header};
use crate::auth::{user::Claims};
use std::{future::{ready, Ready}};
use crate::auth::service::{decode_token, is_valid};


pub struct TokenAuth {
    pub claims: Claims,
}

impl FromRequest for TokenAuth {
    type Error = ActixError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {

        let auth_header = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok());


        if let Some(auth_header) = auth_header{
            println!("{auth_header}");
            if let Some(token) = auth_header.strip_prefix("Bearer "){
                match decode_token(token){
                    Ok(data) => {
                        if is_valid(&data.claims){
                            return ready(Ok(TokenAuth {
                                claims: data.claims,
                            }));
                        }
                    },
                    Err(_) => {}
                }
            }
        }

        ready(Err(actix_web::error::ErrorUnauthorized("Invalid or expired token")))
    }
}
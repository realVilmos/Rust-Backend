use actix_web::{post, get, web, HttpResponse, Responder};
use crate::auth::service::AuthService;
use crate::auth::user::{NewUser, LoginUser};
use regex::Regex;
use crate::auth::extractor::TokenAuth;
use crate::auth::service::generate_jwt;

#[post("/register")]
pub async fn register(auth_service: web::Data<AuthService>, user: web::Json<NewUser>) -> impl Responder {
    let user_data = user.into_inner();

    if user_data.email.trim().is_empty() ||
       user_data.name.trim().is_empty() ||
       user_data.password.trim().is_empty()
    {
        return HttpResponse::BadRequest().body("Invalid input: There are missing fields")
    }

    if user_data.password.len() < 8 {
        return HttpResponse::BadRequest().body("Invalid input: Password has to be at least 8 characters")
    }

    let email_regex = Regex::new(r"^[^@\s]+@[^@\s]+\.[^@\s]+$").expect("invalid regex");
    if !email_regex.is_match(&user_data.email) {
        return HttpResponse::BadRequest().body("Invalid email format.")
    }

    //create web::block so sync database operation doesn't kill the whole async thread. (not to block the async reactor)
    //so we create a new thread where this function finishes and other register requests can go on.
    let service = auth_service.clone();
    let result = web::block(move || service.create_user(user_data)).await;

    match result {
        Ok(Ok(_)) => HttpResponse::Ok().body("User registered successfully."),
        Ok(Err(diesel::result::Error::DatabaseError(kind, _))) => {
            match kind {
                diesel::result::DatabaseErrorKind::UniqueViolation => {
                    HttpResponse::Conflict().body("Email already in use.")
                }
                _ => HttpResponse::InternalServerError().body("Database error."),
            }
        },
        Ok(Err(_)) => {
            HttpResponse::InternalServerError().body("Database error.")
        },
        Err(e) => {
            println!("Blocking task error: {}", e);
            HttpResponse::InternalServerError().body("Unexpected error, please try again.")
        }
    }
}

#[post("/login")]
pub async fn login(auth_service: web::Data<AuthService>, login: web::Json<LoginUser>) -> impl Responder {
    let login_data = login.into_inner();

    let service = auth_service.clone();
    let result = web::block(move || service.login(login_data)).await;

    match result {
        Ok(Some(user)) => {
            match generate_jwt(user.user_id){
                Ok(token) => HttpResponse::Ok().json(serde_json::json!({
                    "token": token,
                    "email": user.email,
                    "name": user.name
                })),
                Err(e) => {
                    println!("JWT generation error: {}", e);
                    HttpResponse::InternalServerError().body("Failed to generate token.")
                }
            }
        }
        Ok(None) => HttpResponse::Unauthorized().body("Invalid email or password."),
        Err(e) => {
            println!("Blocking task error: {}", e);
            HttpResponse::InternalServerError().body("Unexpected error during login.")
        }
    }
}

#[get("/auth_check")]
pub async fn get_authenticated_endpoint(auth: TokenAuth) -> impl Responder {
    HttpResponse::Ok().body(format!("Your user ID is: {}", auth.claims.sub))
}
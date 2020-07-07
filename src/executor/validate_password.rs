use actix_web::{post, HttpServer, web, HttpResponse, Responder};
use log::info;
use pam::Authenticator;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Request {
    username: String,
    password: String,
}

#[derive(Deserialize)] // this is to get users from the database
pub struct Response {
    result: bool,
}

#[post("/validate_password")]
pub async fn validate_password(request: web::Json<Request>) -> impl Responder {
    info!("validating password for {}", request.username);

    if authenticate(&request.username, &request.password) {
        HttpResponse::Ok().json(Response { result: true })
    } else {
        HttpResponse::Ok().json(Response { result: false })
    }
}

pub fn authenticate(username: &str, password: &str) -> bool {
    let mut authenticator =
        Authenticator::with_password("login").expect("Fail to init with client");

    authenticator
        .get_handler()
        .set_credentials(username, password);

    authenticator.authenticate().is_ok()
}

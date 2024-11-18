use serde::{Serialize, Deserialize};
use rocket::{
    http::Status, 
    outcome::IntoOutcome, 
    request::{FromRequest, Outcome}, 
    Request
};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub admin_password: String,
    pub server_secret: String
}

pub struct AdminPass<'r>(pub &'r String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminPass<'r> {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        req.rocket().state::<Config>().map(|config| AdminPass(&config.admin_password)).or_forward(Status::InternalServerError)
    }
}

pub struct ServerSecret<'r>(pub &'r String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ServerSecret<'r> {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        req.rocket().state::<Config>().map(|config| ServerSecret(&config.server_secret)).or_forward(Status::InternalServerError)
    }
}

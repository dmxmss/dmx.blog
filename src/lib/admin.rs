use crate::lib::{
    tokens::{Token, AccessToken},
    errors::AppError,
    config::ServerSecret
};
use rocket::{
    http::Status, request::{FromRequest, Outcome}, Request
};

pub struct Admin;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin {
    type Error = AppError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = req.cookies();
        let secret = req.guard::<ServerSecret>().await.unwrap().0;

        match cookies.get_private(AccessToken::COOKIE_NAME) {
            Some(cookie) => {
                match AccessToken::validate(cookie.value(), &secret) {
                    Ok(()) => Outcome::Success(Admin),
                    Err(_) => Outcome::Forward(Status::Unauthorized)
                }
            },
            None => Outcome::Forward(Status::Unauthorized)
        }
    }
}

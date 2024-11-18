use rocket::{
    request::{Outcome, FromRequest},
    http::Status,
    Request
};
use crate::lib::{
    errors::AppError,
    utils::update_tokens,
    tokens::{Token, RefreshToken},
    config::ServerSecret
};

pub struct Refresh;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Refresh {
    type Error = AppError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = req.cookies();
        let secret = req.guard::<ServerSecret>().await.unwrap().0;

        match cookies.get_private(RefreshToken::COOKIE_NAME) {
            Some(cookie) => {
                match RefreshToken::validate(cookie.value(), secret) {
                    Ok(()) => {
                        match update_tokens(cookies, secret) {
                            Ok(()) => Outcome::Success(Refresh),
                            Err(_) => Outcome::Forward(Status::InternalServerError)
                        }
                    },
                    Err(_) => Outcome::Forward(Status::Unauthorized)
                }
            }, 
            None => {
                Outcome::Forward(Status::Unauthorized)
            }
        }
    }
}

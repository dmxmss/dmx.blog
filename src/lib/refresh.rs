use rocket::{
    request::{Outcome, FromRequest},
    http::Status,
    Request
};
use crate::lib::{
    errors::AppError,
    utils::{update_tokens, get_secret},
    tokens::{Token, RefreshToken}
};

pub struct Refresh;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Refresh {
    type Error = AppError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = req.cookies();
        let secret = get_secret();

        match cookies.get_private(RefreshToken::COOKIE_NAME) {
            Some(cookie) => {
                match RefreshToken::validate(cookie.value(), &secret) {
                    Ok(()) => {
                        match update_tokens(cookies) {
                            Ok(()) => Outcome::Success(Refresh),
                            Err(e) => Outcome::Error((Status::InternalServerError, e))
                        }
                    },
                    Err(e) => Outcome::Error((Status::Unauthorized, e))
                }
            }, 
            None => {
                Outcome::Forward(Status::PermanentRedirect)
            }
        }
    }
}

use crate::lib::{
    tokens::{Token, AccessToken, RefreshToken},
    errors::AppError,
    utils::{get_secret, update_tokens}
};
use rocket::{
    request::{Outcome, FromRequest},
    Request, 
    http::Status
};


pub struct Admin;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin {
    type Error = AppError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = req.cookies();
        let secret = get_secret();

        match cookies.get_private(AccessToken::COOKIE_NAME) {
            Some(cookie) => {
                match AccessToken::validate(cookie.value(), &secret) {
                    Ok(()) => Outcome::Success(Admin),
                    Err(e) => Outcome::Error((Status::Unauthorized, e))
                }
            },
            None => Outcome::Forward(Status::PermanentRedirect)
        }
    }
}

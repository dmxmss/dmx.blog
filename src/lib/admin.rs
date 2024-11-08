use crate::lib::{
    tokens::{Token, AccessToken},
    errors::AppError,
    utils::get_secret
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
        let secret = match get_secret() {
            Ok(s) => s,
            Err(e) => return e.into()
        };

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

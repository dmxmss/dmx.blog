use crate::lib::utils::{set_new_tokens, Claims};
use rocket::{
    request::{Outcome, Request, FromRequest},
    http::Status
};
use jsonwebtoken::{decode, errors::{Error, ErrorKind}, Algorithm, DecodingKey, Validation};

pub struct Admin;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin {
    type Error = jsonwebtoken::errors::Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = req.cookies();
        let secret = std::fs::read_to_string("server_secret").unwrap();

        if let Some(access_token) = cookies.get_private("AccessToken") {
            // access token is present in cookies
            match decode::<Claims>(access_token.value(), &DecodingKey::from_secret(secret.as_ref()), &Validation::new(Algorithm::HS256)) {
                Ok(_) => Outcome::Success(Admin),
                Err(e) => Outcome::Error((Status::Unauthorized, e))
            }
        } else if let Some(refresh_token) = cookies.get_private("RefreshToken") {
            // refresh token is present in cookies
            match decode::<Claims>(refresh_token.value(), &DecodingKey::from_secret(secret.as_ref()), &Validation::new(Algorithm::HS256)) {
                Ok(_) => {
                    set_new_tokens(cookies);

                    Outcome::Success(Admin)
                },
                Err(e) => Outcome::Error((Status::Unauthorized, e))
            }
        } else {
            // there is no token in cookies
            Outcome::Error((Status::Unauthorized, Error::from(ErrorKind::InvalidToken))) 
        }
    }
}

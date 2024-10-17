use crate::lib::utils::{generate_tokens, Claims};
use rocket::{
    request::{Outcome, Request, FromRequest},
    http::{Status, Cookie}
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, errors::{Error, ErrorKind}};
use chrono::offset::Utc;

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

                    let (new_access, new_refresh) = generate_tokens(Utc::now().timestamp());

                    let refresh = Cookie::new("RefreshToken", new_refresh);
                    let access = Cookie::new("AccessToken", new_access);

                    cookies.remove_private(refresh_token);

                    cookies.add_private(refresh);
                    cookies.add_private(access);

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

use jsonwebtoken::{decode, encode, EncodingKey, DecodingKey, Validation, Algorithm};
use time::OffsetDateTime;
use crate::lib::{
    utils::Claims,
    errors::AppError
};

pub trait Token {
    const EXPIRATION_TIME: i64;
    const COOKIE_NAME: &'static str;

    fn validate(token: &str, secret: &str) -> Result<(), AppError> {
        decode::<Claims>(
            token, 
            &DecodingKey::from_secret(secret.as_ref()), 
            &Validation::new(Algorithm::HS256)
        )?;

        Ok(())
    }

    fn encode(secret: &str) -> Result<String, AppError> {
        let exp = Self::get_exp();
        let claims = Claims {
            exp 
        };

        Ok(
            encode(
                &jsonwebtoken::Header::default(),
                &claims, 
                &EncodingKey::from_secret(secret.as_ref())
            )?
        )
    }

    fn get_exp() -> i64 {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        now + Self::EXPIRATION_TIME
    }
}

pub struct RefreshToken;

impl Token for RefreshToken {
    const EXPIRATION_TIME: i64 = 24*60*60; // 24 hours
    const COOKIE_NAME: &'static str = "RefreshToken";
}

pub struct AccessToken;

impl Token for AccessToken {
    const EXPIRATION_TIME: i64 = 60*60; // 1 hour
    const COOKIE_NAME: &'static str = "AccessToken";
}

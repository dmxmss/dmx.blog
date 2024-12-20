use crate::lib::{
    article::Article,
    tokens::{Token, AccessToken, RefreshToken},
    result::Result
};
use serde::{Serialize, Deserialize};
use rocket::http::CookieJar;
use std::{
    fs, path::Path
};
use cookie::Cookie;
use time::OffsetDateTime;

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub exp: i64
}

#[derive(FromForm)]
#[allow(dead_code)]
pub struct LoginData {
    pub password: String
}

pub fn get_articles<P: AsRef<Path>>(path: P) -> Result<Vec<Article>> {
    let data = fs::read_to_string(path)?;

    Ok(serde_json::from_str(data.as_str())?)
}

fn generate_expires_timestamps() -> (OffsetDateTime, OffsetDateTime) {
    let access_exp = OffsetDateTime::from_unix_timestamp(AccessToken::get_exp()).unwrap(); // I assume this operation never fails
    let refresh_exp = OffsetDateTime::from_unix_timestamp(RefreshToken::get_exp()).unwrap();

    (access_exp, refresh_exp)
}

fn generate_token_cookies<'c>(encoded_access: String, encoded_refresh: String) -> (Cookie<'c>, Cookie<'c>) {
    let (access_exp, refresh_exp) = generate_expires_timestamps();

    let access_cookie = Cookie::build((AccessToken::COOKIE_NAME, encoded_access))
        .path("/admin")
        .secure(true)
        .expires(access_exp)
        .http_only(true)
        .build();

    let refresh_cookie = Cookie::build((RefreshToken::COOKIE_NAME, encoded_refresh))
        .path("/refresh")
        .secure(true)
        .expires(refresh_exp)
        .http_only(true)
        .build();

    (refresh_cookie, access_cookie)
}

fn generate_tokens(secret: &str) -> Result<(String, String)> {
    let access = AccessToken::encode(secret)?;
    let refresh = RefreshToken::encode(secret)?;

    Ok((access, refresh))
}

fn write_tokens_to_cookies(access_token: String, refresh_token: String, cookies: &CookieJar) {
    let (access_cookie, refresh_cookie) = generate_token_cookies(access_token, refresh_token);

    cookies.add_private(access_cookie);
    cookies.add_private(refresh_cookie);
}

pub fn update_tokens(cookies: &CookieJar, secret: &str) -> Result<()> {
    let (access_token, refresh_token) = generate_tokens(secret)?;

    write_tokens_to_cookies(access_token, refresh_token, cookies);

    Ok(())
}

use crate::lib::article::Article;
use serde::{Serialize, Deserialize};
use rocket::{
    form::{self, Error},
    http::CookieJar
};
use std::{
    path::Path,
    fs::{self, File},
    io::BufReader
};
use jsonwebtoken::{
    self,
    encode,
    EncodingKey,
    Header
};
use cookie::Cookie;
use time::{Duration, OffsetDateTime};

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub exp: i64,
    pub iat: i64,
}

#[derive(FromForm)]
#[allow(dead_code)]
pub struct LoginData {
    #[field(validate = authenticate())]
    pub password: String
}

pub fn get_articles<P: AsRef<Path>>(path: P) -> Vec<Article> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).unwrap()
}

fn authenticate<'v>(password: &str) -> form::Result<'v, ()> {
    if password.trim() != fs::read_to_string("admin").unwrap().trim() {
        Err(Error::validation("invalid admin password"))?;
    }

    Ok(())
}

fn generate_tokens(iat: i64) -> (String, String) {
    let secret = fs::read_to_string("server_secret").unwrap();

    let access_exp: OffsetDateTime = OffsetDateTime::from_unix_timestamp(iat).unwrap() + Duration::SECOND*24*60*60; // 24h
    let refresh_exp: OffsetDateTime = OffsetDateTime::from_unix_timestamp(iat).unwrap() + Duration::SECOND*7*24*60*60; // 7d


    let access = encode(&Header::default(), &Claims {exp: access_exp.unix_timestamp(), iat}, &EncodingKey::from_secret(secret.as_ref())).unwrap();
    let refresh = encode(&Header::default(), &Claims {exp: refresh_exp.unix_timestamp(), iat}, &EncodingKey::from_secret(secret.as_ref())).unwrap();

    (access, refresh)
}

fn generate_token_cookies<'c>() -> (Cookie<'c>, Cookie<'c>) {
    let now = OffsetDateTime::now_utc();
    let (new_access, new_refresh) = generate_tokens(now.unix_timestamp());

    let access_exp: OffsetDateTime = now + Duration::SECOND*24*60*60; // 24h
    let refresh_exp = now + Duration::SECOND*7*24*60*60; // 7d

    let access_cookie = Cookie::build(("AccessToken", new_access))
        .path("/admin")
        .secure(true)
        .expires(access_exp)
        .http_only(true)
        .build();

    let refresh_cookie = Cookie::build(("RefreshToken", new_refresh))
        .path("/admin")
        .secure(true)
        .expires(refresh_exp)
        .http_only(true)
        .build();

    (refresh_cookie, access_cookie)
}

pub fn set_new_tokens(jar: &CookieJar<'_>) {
    let (refresh_cookie, access_cookie) = generate_token_cookies();

    jar.add_private(refresh_cookie);
    jar.add_private(access_cookie);
}

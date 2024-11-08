use crate::lib::{
    article::{Article, NewArticle},
    errors::AppError,
    tokens::{Token, AccessToken, RefreshToken}
};
use serde::{Serialize, Deserialize};
use rocket::{
    form::{self, Error},
    http::CookieJar
};
use std::{
    fs::{self, File}, io::{BufReader, Write}, path::Path
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
    #[field(validate = check_pass())]
    pub password: String
}

pub fn get_articles<P: AsRef<Path>>(path: P) -> Vec<Article> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).unwrap()
}

pub fn create_article<P: AsRef<Path>>(path: P, article: NewArticle) -> u64 {
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(&file);

    let mut articles: Vec<Article> = serde_json::from_reader(reader).unwrap();

    let mut file = File::create(&path).unwrap();

    let id = articles.iter().map(|a| a.id).max().unwrap() + 1;
    let article = Article::new(id, article.name, article.contents);
    articles.push(article);

    file.write_all(serde_json::to_string(&articles).unwrap().as_bytes()).unwrap();

    id
}

fn check_pass<'v>(password: &str) -> form::Result<'v, ()> {
    if password.trim() != fs::read_to_string("admin").unwrap().trim() {
        Err(Error::validation("invalid admin password"))?;
    }

    Ok(())
}

fn generate_expires_timestamps() -> (OffsetDateTime, OffsetDateTime) {
    let access_exp = OffsetDateTime::from_unix_timestamp(AccessToken::get_exp()).unwrap();
    let refresh_exp = OffsetDateTime::from_unix_timestamp(RefreshToken::get_exp()).unwrap();

    (access_exp, refresh_exp)
}

fn generate_token_cookies<'c>(encoded_access: String, encoded_refresh: String) -> Result<(Cookie<'c>, Cookie<'c>), AppError> {
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

    Ok((refresh_cookie, access_cookie))
}

pub fn get_secret() -> String {
    std::fs::read_to_string("server_secret").unwrap()
}

pub fn generate_tokens() -> Result<(String, String), AppError> {
    let secret = get_secret();

    let access = AccessToken::encode(&secret)?;
    let refresh = RefreshToken::encode(&secret)?;

    Ok((access, refresh))
}

pub fn write_tokens_to_cookies(access_token: String, refresh_token: String, cookies: &CookieJar) -> Result<(), AppError> {
    let (access_cookie, refresh_cookie) = generate_token_cookies(access_token, refresh_token)?;

    cookies.add_private(access_cookie);
    cookies.add_private(refresh_cookie);

    Ok(())
}

pub fn update_tokens(cookies: &CookieJar) -> Result<(), AppError> {
    let (access_token, refresh_token) = generate_tokens()?;

    write_tokens_to_cookies(access_token, refresh_token, cookies)
}

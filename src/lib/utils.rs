use crate::lib::{
    article::{Article, NewArticle},
    tokens::{Token, AccessToken, RefreshToken},
    result::Result
};
use serde::{Serialize, Deserialize};
use rocket::{
    form,
    http::{Status, CookieJar}
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

pub fn get_articles<P: AsRef<Path>>(path: P) -> Result<Vec<Article>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(serde_json::from_reader(reader)?)
}

pub fn get_article<P: AsRef<Path>>(path: P, id: u64) -> Result<Option<Article>> {
    let articles = get_articles(path)?;

    Ok(articles.into_iter().find(|article| article.id == id))
}

pub fn create_article<P: AsRef<Path>>(path: P, article: NewArticle) -> Result<u64> {
    let mut articles = get_articles(&path)?;

    let id = articles.iter().map(|a| a.id).max().unwrap() + 1;
    let article = Article::new(id, article.name, article.contents);
    articles.push(article);

    write_to_db(&path, articles)?;

    Ok(id)
}

pub fn delete_article_by_id<P: AsRef<Path>>(path: P, id: u64) -> Result<()> {
    let mut articles = get_articles(&path)?;

    if let Some(article) = get_article(&path, id)? {
        articles.retain(|a| a.id != article.id);
    }

    
    write_to_db(&path, articles)?;

    Ok(())
}

pub fn update_article<P: AsRef<Path>>(path: P, id: u64, article: NewArticle) -> Result<()> {
    delete_article_by_id(&path, id)?;

    let mut articles = get_articles(&path)?;
    let article = Article::new(id, article.name, article.contents);
    articles.push(article);

    write_to_db(&path, articles)?;

    Ok(())
}

fn write_to_db<P: AsRef<Path>>(path: P, articles: Vec<Article>) -> Result<()> {
    let mut file = File::create(&path)?;
    
    file.write_all(serde_json::to_string(&articles).unwrap().as_bytes()).unwrap();

    Ok(())
}

fn check_pass<'v>(input_pass: &str) -> form::Result<'v, ()> {
    let pass = match fs::read_to_string("admin") {
        Ok(p) => p,
        Err(e) => return Err(form::error::ErrorKind::Custom(Status::InternalServerError, Box::new(e)).into())
    };
    
    if input_pass.trim() != pass.trim() {
        Err(form::Error::validation("invalid admin password"))?;
    }

    Ok(())
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

pub fn get_secret() -> Result<String> {
    Ok(std::fs::read_to_string("server_secret")?)
}

pub fn generate_tokens() -> Result<(String, String)> {
    let secret = get_secret()?;

    let access = AccessToken::encode(&secret)?;
    let refresh = RefreshToken::encode(&secret)?;

    Ok((access, refresh))
}

pub fn write_tokens_to_cookies(access_token: String, refresh_token: String, cookies: &CookieJar) {
    let (access_cookie, refresh_cookie) = generate_token_cookies(access_token, refresh_token);

    cookies.add_private(access_cookie);
    cookies.add_private(refresh_cookie);
}

pub fn update_tokens(cookies: &CookieJar) -> Result<()> {
    let (access_token, refresh_token) = generate_tokens()?;

    write_tokens_to_cookies(access_token, refresh_token, cookies);

    Ok(())
}

use crate::lib::article::Article;
use serde::{Serialize, Deserialize};
use rocket::form::{self, Error};
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

pub fn generate_tokens(iat: i64) -> (String, String) {
    let secret = fs::read_to_string("server_secret").unwrap();
    let now = chrono::Utc::now().timestamp();

    let access_exp = 1 + now; // 24h 
    let refresh_exp = 7*24*60*60 + now; // 7d 

    let access = encode(&Header::default(), &Claims {exp: access_exp, iat}, &EncodingKey::from_secret(secret.as_ref())).unwrap();
    let refresh = encode(&Header::default(), &Claims {exp: refresh_exp, iat}, &EncodingKey::from_secret(secret.as_ref())).unwrap();

    (access, refresh)
}

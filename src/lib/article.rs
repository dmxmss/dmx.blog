use serde::{Deserialize, Serialize};
use rocket::form::FromForm;
use chrono::Utc;

#[derive(Serialize, Deserialize)]
pub struct Article {
    pub id: u64,
    pub name: String,
    pub contents: String,
    pub pub_date: String,
    pub edit_date: String
}

impl Article {
    pub fn new(id: u64, name: String, contents: String) -> Article {
        let pub_date = Utc::now().format("%Y-%m-%d %H:%M").to_string();
        let edit_date = pub_date.clone();

        Article {
            id,
            name,
            contents,
            pub_date,
            edit_date
        }
    }
}

#[derive(FromForm)]
pub struct NewArticle {
    pub name: String, 
    pub contents: String
}

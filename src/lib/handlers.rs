use rocket_dyn_templates::{Template, context};
use std::fs;

use crate::lib::article::Article;

#[get("/")]
pub fn index() -> Template {
    let articles: Vec<Article> = serde_json::from_str(fs::read_to_string("articles.json").unwrap().as_str()).unwrap();

    Template::render("index", context! {articles: articles})
}

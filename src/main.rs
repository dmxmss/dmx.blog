#[macro_use] extern crate rocket;

use serde::{Deserialize, Serialize};
use rocket_dyn_templates::{Template, context};
use chrono::offset::Utc;
use std::fs;

#[derive(Serialize)]
struct Context {
    articles: Vec<Article>
}

#[derive(Serialize, Deserialize)]
struct Article {
    id: u64,
    name: String,
    contents: String,
    pub_date: String,
    edit_date: String
}

impl Article {
    fn new(id: u64, name: String, contents: String) -> Article {
        Article {
            id, 
            name, 
            contents, 
            pub_date: Utc::now().to_string(), 
            edit_date: Utc::now().to_string()
        }
    }
}

#[get("/")]
fn index() -> Template {
    // let context = Context {
    //     articles: vec![
    //         Article::new(1, String::from("First article"), String::from("Hello world")),
    //         Article::new(2, String::from("Second article"), String::from("Hello world")),
    //         Article::new(3, String::from("Third article"), String::from("Hello world"))
    // };

    let articles: Vec<Article> = serde_json::from_str(fs::read_to_string("articles.json").unwrap().as_str()).unwrap();

    Template::render("index", context! {articles: articles})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}

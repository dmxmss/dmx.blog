#[macro_use] extern crate rocket;

use serde::Serialize;
use rocket_dyn_templates::Template;

#[derive(Serialize)]
struct Context {
    articles: Vec<Article>
}

#[derive(Serialize)]
struct Article {
    id: u64,
    name: String,
    contents: String
}

#[get("/")]
fn index() -> Template {
    let context = Context {
        articles: vec![
            Article {
                id: 1, 
                name: String::from("First article"), 
                contents: String::from("Hello, world")
            }, 
            Article {
                id: 2, 
                name: String::from("Second article"), 
                contents: String::from("Hello, world")
            }, 
            Article {
                id: 3, 
                name: String::from("Third article"), 
                contents: String::from("Hello, world")
            }, 

        ]
    };

    Template::render("index", context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}

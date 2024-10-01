use rocket_dyn_templates::{Template, context};

use crate::lib::utils::get_articles;

#[get("/")]
pub fn index() -> Template {
    let articles = get_articles("articles.json");

    Template::render("index", context! { articles: articles })
}

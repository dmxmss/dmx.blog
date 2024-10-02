use rocket_dyn_templates::{Template, context};
use crate::lib::utils::get_articles;

#[get("/")]
pub fn index() -> Template {
    let articles = get_articles("articles.json");

    Template::render("index", context! { articles: articles })
}

#[get("/article/<id>")] 
pub fn article(id: u64) -> Option<Template> {
    let articles = get_articles("articles.json");
    if let Some(article) = articles.into_iter().find(|article| article.id == id) {
        Some(Template::render("article", context! { article: article }))
    } else {
        None
    }
}

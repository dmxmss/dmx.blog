use rocket_dyn_templates::{Template, context};
use rocket::{
    form::Form, 
    fs::NamedFile, 
    http::CookieJar, 
    response::Redirect
};
use crate::lib::{
    utils::{set_new_tokens, get_articles, LoginData},
    admin::Admin
};

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

#[get("/new")]
pub async fn get_new_article_form() -> Option<NamedFile> {
    NamedFile::open("static/new_article.html").await.ok()
}

#[get("/login")]
pub async fn get_admin_login_form() -> Template {
    Template::render("login", context! { wrong_pass: false })
}

#[post("/login", data = "<_data>")]
pub async fn login(jar: &CookieJar<'_>, _data: Form<LoginData>) -> Redirect {
    set_new_tokens(jar);
    Redirect::to(uri!("/admin")) 
}

#[get("/admin")]
pub async fn admin_page(_admin: Admin) -> Template {
    let articles = get_articles("articles.json");

    Template::render("index", context! { articles: articles })
}

#[catch(401)]
pub fn unauthorized() -> Redirect {
    Redirect::to(uri!("/login"))
}

#[catch(422)]
pub fn wrong_password() -> Template {
    Template::render("login", context! { wrong_pass: true })
}

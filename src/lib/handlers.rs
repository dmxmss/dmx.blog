use rocket_dyn_templates::{Template, context};
use rocket::{
    form::Form, 
    fs::NamedFile, 
    http::{Status, CookieJar}, 
    response::Redirect,
    Request
};
use crate::lib::{
    utils::{update_tokens, get_articles, LoginData},
    admin::Admin,
    refresh::Refresh
};

#[get("/")]
pub fn index() -> Template {
    let articles = get_articles("articles.json");

    Template::render("index", context! { articles: articles })
}

#[get("/article/<id>")] 
pub fn article(id: u64) -> Option<Template> {
    let articles = get_articles("articles.json");

    articles.into_iter().find(|article| article.id == id).map(|article| Template::render("article", context! { article: article }))
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
pub async fn login(jar: &CookieJar<'_>, _data: Form<LoginData>) -> Result<Redirect, (Status, &'static str)> {
    if update_tokens(jar).is_err() {
        return Err((Status::InternalServerError, "Internal server error"));
    }

    Ok(Redirect::to(uri!("/admin")))
}

#[get("/admin")]
pub async fn admin(admin: Option<Admin>) -> Result<Template, Redirect> {
    if admin.is_none() {
        return Err(Redirect::to(uri!("/refresh")));
    }

    let articles = get_articles("articles.json");

    Ok(Template::render("index", context! { articles: articles }))
}

#[get("/refresh")]
pub fn refresh(refresh: Option<Refresh>) -> Redirect {
    if refresh.is_none() {
        return Redirect::to(uri!("/login"));
    }

    Redirect::to(uri!("/admin"))
}

#[get("/admin", rank = 3)]
pub fn admin_redirect() -> Redirect {
    Redirect::to(uri!("/login"))
}


#[catch(401)]
pub fn unauthorized() -> Redirect {
    Redirect::to(uri!("/login"))
}

#[catch(422)]
pub fn wrong_password() -> Template {
    Template::render("login", context! { wrong_pass: true })
}

#[catch(500)]
pub fn internal_server_error() -> Redirect {
    Redirect::to(uri!("/login"))
}

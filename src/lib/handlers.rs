use rocket_dyn_templates::{Template, context};
use rocket::{
    form::Form, 
    fs::NamedFile, 
    http::{Status, CookieJar}, 
    response::Redirect,
};
use crate::lib::{
    utils::{create_article, update_tokens, get_articles, LoginData},
    admin::Admin,
    refresh::Refresh,
    article::NewArticle
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
pub async fn article_form(_admin: Admin) -> Option<NamedFile> {
    NamedFile::open("static/new_article.html").await.ok()
}

#[post("/new", data = "<article>")]
pub fn new_article(_admin: Admin, article: Form<NewArticle>) -> Redirect {
    let id = create_article("articles.json", article.into_inner()); // Add error handling
    Redirect::to(uri!(article(id)))
}

#[get("/login")]
pub async fn login_form() -> Template {
    Template::render("login", context! { wrong_pass: false })
}

#[post("/login", data = "<_data>")]
pub async fn login(jar: &CookieJar<'_>, _data: Form<LoginData>) -> Result<Redirect, Status> {
    if update_tokens(jar).is_err() {
        return Err(Status::InternalServerError);
    }

    Ok(Redirect::to(uri!("/admin")))
}

#[get("/admin")]
pub async fn admin(_admin: Admin) -> Template {
    let articles = get_articles("articles.json");

    Template::render("dashboard", context! { articles: articles })
}

#[get("/admin", rank = 2)]
pub fn not_admin() -> Redirect {
    Redirect::to(uri!("/refresh"))
}

#[get("/refresh")]
pub fn refresh(_refresh: Refresh) -> Redirect {
    Redirect::to(uri!("/admin"))
}

#[get("/refresh", rank = 2)]
pub fn fail_refresh() -> Redirect {
    Redirect::to(uri!(login_form()))
}

#[catch(401)]
pub async fn unauthorized() -> Option<NamedFile> {
    NamedFile::open("static/unauthorized.html").await.ok() 
}

#[catch(422)]
pub fn wrong_password() -> Template {
    Template::render("login", context! { wrong_pass: true })
}

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
pub fn index() -> Result<Template, Status> {
    let articles = get_articles("articles.json")?;

    Ok(Template::render("index", context! { articles: articles }))
}

#[get("/article/<id>")] 
pub fn article(id: u64) -> Result<Template, Status> {
    let articles = get_articles("articles.json")?;

    articles
        .into_iter()
        .find(|article| article.id == id)
        .map(|article| Template::render("article", context! { article: article }))
        .ok_or(Status::NotFound)
}

#[get("/new")]
pub async fn article_form(_admin: Admin) -> Option<NamedFile> {
    NamedFile::open("static/new_article.html").await.ok()
}

#[post("/new", data = "<article>")]
pub fn new_article(_admin: Admin, article: Form<NewArticle>) -> Result<Redirect, Status> {
    let id = create_article("articles.json", article.into_inner())?;

    Ok(Redirect::to(uri!(article(id))))
}

#[get("/login")]
pub async fn login_form() -> Template {
    Template::render("login", context! { wrong_pass: false })
}

#[post("/login", data = "<_data>")]
pub async fn login(cookies: &CookieJar<'_>, _data: Form<LoginData>) -> Result<Redirect, Status> {
    update_tokens(cookies)?;

    Ok(Redirect::to(uri!("/admin")))
}

#[get("/admin")]
pub async fn admin(_admin: Admin) -> Result<Template, Status> {
    let articles = get_articles("articles.json")?;

    Ok(Template::render("dashboard", context! { articles: articles }))
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

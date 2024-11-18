use std::sync::Mutex;
use rocket_dyn_templates::{Template, context};
use rocket::{
    form::Form, fs::NamedFile, http::{CookieJar, Status}, response::Redirect, State,
};
use crate::lib::{
    utils::{update_tokens, LoginData},
    admin::Admin,
    refresh::Refresh,
    article::NewArticle,
    db::Cursor,
    config::{AdminPass, ServerSecret}
};

#[get("/")]
pub fn index(db: &State<Mutex<Cursor>>) -> Result<Template, Status> {
    let articles = &db.lock().unwrap().articles;

    Ok(Template::render("index", context! { articles: articles }))
}

#[get("/article/<id>")] 
pub fn article(id: u64, db: &State<Mutex<Cursor>>) -> Result<Template, Status> {
    let article = db.lock().unwrap().get_article(id).ok_or(Status::NotFound)?;
    
    Ok(Template::render("article", context! {article: article}))
}

#[get("/new")]
pub async fn article_form(_admin: Admin) -> Option<NamedFile> {
    NamedFile::open("static/new_article.html").await.ok()
}

#[post("/new", data = "<article>")]
pub fn new(_admin: Admin, article: Form<NewArticle>, db: &State<Mutex<Cursor>>) -> Result<Redirect, Status> {
    let id = db.lock().unwrap().create_article(article.into_inner())?;

    Ok(Redirect::to(uri!(article(id))))
}

#[get("/delete/<id>")]
pub fn delete(_admin: Admin, id: u64, db: &State<Mutex<Cursor>>) -> Result<Redirect, Status> {
    db.lock().unwrap().delete_article(id)?;

    Ok(Redirect::to(uri!("/admin")))
}

#[get("/edit/<id>")]
pub fn edit_article_form(_admin: Admin, id: u64, db: &State<Mutex<Cursor>>) -> Result<Template, Status> {
    let article = db.lock().unwrap().get_article(id).ok_or(Status::NotFound)?;

    Ok(Template::render("edit", context! { article: article }))
}

#[post("/edit/<id>", data = "<article>")]
pub fn edit(_admin: Admin, id: u64, article: Form<NewArticle>, db: &State<Mutex<Cursor>>) -> Result<Redirect, Status> {
    db.lock().unwrap().update_article(id, article.into_inner())?;

    Ok(Redirect::to(uri!(article(id))))
}

#[get("/login")]
pub async fn login_form() -> Template {
    Template::render("login", context! { wrong_pass: false })
}

#[post("/login", data = "<data>")]
pub async fn login(cookies: &CookieJar<'_>, data: Form<LoginData>, admin_pass: AdminPass<'_>, secret: ServerSecret<'_>) -> Result<Redirect, Status> {
    if *admin_pass.0 != data.into_inner().password {
        return Err(Status::Unauthorized);
    }

    update_tokens(cookies, secret.0)?;

    Ok(Redirect::to(uri!("/admin")))
}

#[get("/admin")]
pub async fn admin(_admin: Admin, db: &State<Mutex<Cursor>>) -> Result<Template, Status> {
    let articles = &db.lock().unwrap().articles;

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

#[catch(401)]
pub fn wrong_password() -> Template {
    Template::render("login", context! { wrong_pass: true })
}

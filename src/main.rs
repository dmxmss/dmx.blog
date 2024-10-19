#[macro_use] extern crate rocket;

use rocket_dyn_templates::Template;

mod lib;
use lib::handlers::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, article, get_new_article_form, get_admin_login_form, admin_page, login])
        .register("/", catchers![unauthorized, wrong_password])
        .attach(Template::fairing())
}

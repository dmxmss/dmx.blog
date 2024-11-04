#[macro_use] extern crate rocket;

use rocket_dyn_templates::Template;

mod lib;
use lib::handlers::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, article, get_new_article_form, get_admin_login_form, admin, refresh, login])
        .register("/", catchers![unauthorized, wrong_password])
        .register("/login", catchers![internal_server_error])
        .attach(Template::fairing())
}

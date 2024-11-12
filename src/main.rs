#[macro_use] extern crate rocket;

use rocket_dyn_templates::Template;

mod lib;
use lib::handlers::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, article, login_form, admin, not_admin, refresh, fail_refresh, login])
        .mount("/admin", routes![article_form, new_article, delete_article, edit_article])
        .register("/admin", catchers![unauthorized])
        .register("/login", catchers![wrong_password])
        .attach(Template::fairing())
}

#[macro_use] extern crate rocket;

use rocket_dyn_templates::Template;

mod lib;
use lib::handlers::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, article, new_article_form])
        .attach(Template::fairing())
}

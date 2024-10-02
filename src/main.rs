#[macro_use] extern crate rocket;

use rocket_dyn_templates::Template;

mod lib;
use lib::handlers::{index, article};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, article])
        .attach(Template::fairing())
}

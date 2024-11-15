#[macro_use] extern crate rocket;

use rocket_dyn_templates::Template;

mod lib;
use lib::fairings::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(unauthorized_actions())
        .attach(authorized_actions())
        .attach(authorization())
        .attach(catchers())
        .attach(fail_auth())
        .attach(init_db("articles.json"))
        .attach(Template::fairing())
}

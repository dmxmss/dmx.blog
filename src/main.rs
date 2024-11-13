#[macro_use] extern crate rocket;

use rocket_dyn_templates::Template;

mod lib;
use lib::handlers::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(unauthorized_actions())
        .attach(authorized_actions())
        .attach(authorization())
        .attach(fail_auth())
        .attach(catchers())
        .attach(Template::fairing())
}

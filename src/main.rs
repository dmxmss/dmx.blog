#[macro_use] extern crate rocket;

use rocket::fairing::AdHoc ;
use rocket_dyn_templates::Template;

mod lib;
use lib::{
    fairings::*,
    config::Config
};
use figment::{
    Figment,
    providers::{Toml, Format}
};

#[launch]
fn rocket() -> _ {
    let figment = Figment::from(rocket::config::Config::default())
        .merge(Toml::file("Rocket.toml").nested())
        .merge(Toml::file("App.toml").nested());

    rocket::custom(figment)
        .attach(unauthorized_actions())
        .attach(authorized_actions())
        .attach(authorization())
        .attach(catchers())
        .attach(fail_auth())
        .attach(init_db("articles.json"))
        .attach(Template::fairing())
        .attach(AdHoc::config::<Config>())
}

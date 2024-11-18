use crate::lib::{
    handlers::*,
    db::Cursor
};
use std::{
    path::Path,
    sync::Mutex
};

pub fn unauthorized_actions() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Unauthorized", |rocket| async {
        rocket.mount("/", routes![index, article, login_form, login])
    })  
}

pub fn authorization() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Authorization", |rocket| async {
        rocket.mount("/", routes![admin, refresh])
    })  
}

pub fn fail_auth() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Fail authorization", |rocket| async {
        rocket.mount("/", routes![not_admin, fail_refresh])
    })  
}

pub fn authorized_actions() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Authorized actions", |rocket| async {
        rocket.mount("/admin", routes![article_form, new, delete, edit_article_form, edit])
    })  
}

pub fn catchers() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Catchers", |rocket| async {
        rocket.register("/admin", catchers![unauthorized])
              .register("/login", catchers![wrong_password])
    })  
}

pub fn init_db<P: AsRef<Path> + Send + 'static>(path: P) -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::try_on_ignite("Init database", |rocket| async {
        match Cursor::new(path) {
            Ok(cursor) => {
                Ok(rocket.manage(Mutex::new(cursor)))
            },
            Err(e) =>  {
                println!("{e}");
                Err(rocket)
            }
        }
    })  
}

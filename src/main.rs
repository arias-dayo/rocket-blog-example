#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
extern crate dotenv;

mod routes;
mod db;
mod models;
mod schema;

use rocket_contrib::templates::{Template};
use rocket_contrib::serve::StaticFiles;
use routes::*;

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage(db::connect())
        .mount("/static", StaticFiles::from("static"))
        .mount("/", routes![contents::index, contents::posts, 
                            authorize::login, authorize::admin])
        .register(catchers![error::not_found, error::server_error])
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
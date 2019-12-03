#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
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
        .mount("/static", StaticFiles::from("static"))
        .mount("/", routes![contents::index, contents::posts, 
                            authorize::login, authorize::admin])
        .register(catchers![error::not_found])
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
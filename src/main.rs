#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

mod routes;
mod db;
mod models;
mod schema;

use rocket_contrib::templates::{Template};
use rocket_contrib::serve::StaticFiles;
use routes::*;
use db::*;

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/static", StaticFiles::from("static"))
        .mount("/", routes![index, posts])
        .register(catchers![not_found])
        .attach(Template::fairing())
        .attach(LogsDbConn::fairing())
}

fn main() {
    rocket().launch();
}
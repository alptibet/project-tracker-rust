use dotenv::dotenv;

#[macro_use]
extern crate rocket;

mod controllers;
mod db;
mod errors;
mod models;
mod routes;

use routes::contractors::*;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .attach(db::init())
        .mount("/api", routes![get_contractors, get_one_contractor])
}

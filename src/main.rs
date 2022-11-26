use dotenv::dotenv;

#[macro_use]
extern crate rocket;

mod controllers;
mod db;
mod errors;
mod models;
mod routes;

use routes::contractor::*;
use routes::user::*;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .attach(db::init())
        .mount("/", routes![signup, login, logout])
        .mount(
            "/api/contractors",
            routes![
                get_contractors,
                get_one_contractor,
                insert_one_contractor,
                delete_one_contractor,
                update_one_contractor
            ],
        )
        .mount("/api/users", routes![get_users, get_one_user])
}

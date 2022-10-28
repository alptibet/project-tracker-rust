use dotenv::dotenv;

#[macro_use]
extern crate rocket;

mod db;
mod routes;

use routes::projects::*;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().attach(db::init()).mount("/api", routes![get_all_projects, get_one])
}

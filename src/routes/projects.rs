use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(crate="rocket::serde")]

pub struct Project<'a> {
    pub name: &'a str,
    pub duration: u8,
    pub address: &'a str,
}

#[get("/projects/get-all-projects")]
pub fn get_all_projects<'a>() -> Json<Project<'a>> {
    Json(Project{
        name: "TEST",
        duration: 12,
        address: "TEST ADDRESS"
    })
}


#[get("/projects/get-one")]
pub fn get_one<'a>() -> Json<Project<'a>>{
    Json(Project{
        name:"TEST2",
        duration:11,
        address:"TEST @ ADDER",
    })
}


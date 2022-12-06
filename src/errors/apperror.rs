use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket::serde::Serialize;
use std::io::Cursor;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AppError {
    pub code: u16,
    pub message: String,
}

impl AppError {
    pub fn build(code: u16) -> AppError {
        let mes = match code {
            400 => "Bad Request".to_string(),
            401 => "Unauthorized access".to_string(),
            404 => "Not Found or resource does not exit".to_string(),
            500 => "Internal server error".to_string(),
            _ => "Something went wrong".to_string(),
        };
        AppError { code, message: mes }
    }
}

impl<'r> Responder<'r, 'static> for AppError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let body = serde_json::to_string(&self).unwrap();
        Response::build()
            .sized_body(body.len(), Cursor::new(body))
            .header(ContentType::JSON)
            .status(Status::new(self.code))
            .ok()
    }
}

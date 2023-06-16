

use super::{HttpResponse, ResponseError};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ActixCustomError {
    pub error_code: u16,
    pub message: String,
}

impl ActixCustomError {
    pub fn new(error_code:u16, message: String) -> Self {
        Self { error_code, message }
    }
}

impl std::fmt::Display for ActixCustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for ActixCustomError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self)
    }
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::from_u16(self.error_code).unwrap()
    }
}
use std::error::Error;

use actix_web::HttpResponse;
use actix_web::web::Json;
use serde::Serialize;

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl ErrorResponse {
    pub fn new(error: String) -> Self {
        ErrorResponse {
            error
        }
    }
}

#[derive(Serialize)]
pub struct BoolResponse {
    success: bool,
}

impl BoolResponse {
    pub fn new(success: bool) -> Self {
        BoolResponse {
            success
        }
    }
}

pub fn generate_response<T>(value: Result<T, Box<dyn Error>>) -> HttpResponse
    where
        T: Serialize
{
    match value {
        Ok(value) => {
            HttpResponse::Ok()
                .content_type("application/json")
                .json(value)
        }
        Err(error) => HttpResponse::BadRequest()
            .content_type("application/json")
            .json(Json(ErrorResponse::new(error.to_string())))
    }
}


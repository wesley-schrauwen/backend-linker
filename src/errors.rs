// use std::fmt;
// use std::fmt::Formatter;
// use actix_web::http::StatusCode;
// use actix_web::{HttpResponse, ResponseError};
// use actix_web::body::BoxBody;
// use diesel::result::Error as DieselError;
// use serde_json::json;

// #[derive(Debug, Deserialize)]
// pub struct ApiError {
//     pub status_code: u16,
//     pub message: String,
// }
//
// impl ApiError {
//     pub fn new (status_code: u16, message: String) -> ApiError {
//         ApiError { status_code, message }
//     }
// }
//
// impl fmt::Display for ApiError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         f.write_str(self.message.as_str())
//     }
// }
//
// impl From<DieselError> for ApiError {
//     fn from(diesel_error: DieselError) -> ApiError {
//         match diesel_error {
//             DieselError::DatabaseError(_, error) => ApiError::new(409, error.message().to_string()),
//             DieselError::NotFound() => ApiError::new(404, "Record not found".to_string()),
//             dieselError => ApiError::new(500, format!("Diesel error: {}", dieselError))
//         }
//     }
// }
//
// impl ResponseError for ApiError {
//     fn error_response(&self) -> HttpResponse {
//         let status_code = match StatusCode::from_u16(self.status_code) {
//             Ok(status_code) => status_code,
//             Err(_) => StatusCode::INTERNAL_SERVER_ERROR
//         };
//
//         let message = match status_code.as_u16() < 500 {
//             true => self.message.clone(),
//             false => "Internal server error".to_string()
//         };
//
//         HttpResponse::build(status_code).json(json!({ "message": message }))
//     }
// }
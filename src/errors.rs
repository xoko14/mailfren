use std::fmt::Display;

use axum::{response::IntoResponse, http::StatusCode, Json};
use serde::Serialize;


pub type MailfrenResult<T> = Result<T, MailfrenError>;

#[derive(Debug)]
pub enum MailfrenError{
    InternalServerError,
    NotFound,
    EntityNotFound(String, i64)
}

#[derive(Serialize)]
pub struct ProblemDetails{
    code: u16,
    title: String,
    description: String
}

impl ProblemDetails{
    pub fn new(code: u16, title: &str, description: &str) -> Self{
        Self { code, title: title.to_owned(), description: description.to_owned() }
    }
}

impl MailfrenError{
    pub fn into_problemdetails(&self) -> ProblemDetails{
        match self {
            MailfrenError::EntityNotFound(entity, id) =>
                ProblemDetails::new(
                    404,
                    "Entity not found",
                    &format!("{} with id {}", entity, id)
                ),
            MailfrenError::InternalServerError => 
                ProblemDetails::new(
                    500, "Internal server error", ""),
            MailfrenError::NotFound =>
                ProblemDetails::new(404, "Not found", "Resource not found"),
        }
    }
}

impl Display for MailfrenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {:?}", self)
    }
}

impl IntoResponse for MailfrenError {
    fn into_response(self) -> axum::response::Response {
        let problem_details = self.into_problemdetails();
        (
            StatusCode::from_u16(problem_details.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            Json(problem_details)
        ).into_response()
    }
}

impl From<sqlx::Error> for MailfrenError{
    fn from(value: sqlx::Error) -> Self {
        match value {
            _ => Self::InternalServerError
        }
    }
}
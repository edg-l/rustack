use std::fmt::Debug;

use actix_web::http::StatusCode;
use actix_web::web::HttpResponse;
use actix_web::ResponseError;
use askama::Template;
use thiserror::Error;
use validator::ValidationErrors;
use crate::views::Error as ViewError;
use tracing::error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("validation errors {0}")]
    ValidationErrors(#[from] ValidationErrors),
    #[error("actix web error {0}")]
    ActixError(#[from] actix_web::Error),
    #[error("askama error {0}")]
    TemplateError(#[from] askama::Error),
    #[error("database error {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("not found")]
    NotFound,
    #[error("missing permision: {0}")]
    MissingPermission(String),
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::ValidationErrors(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    #[tracing::instrument]
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::ValidationErrors(e) => {
                error!("Validaton error: {:?}", e.errors());
                HttpResponse::BadRequest().json(e.errors())
            }
            Self::NotFound => {
                let template = ViewError::new("Error 404", 404, "The page you visited was not found.");
                let body = template.render().unwrap();
                HttpResponse::NotFound().content_type("text/html").body(body)
            }
            Self::MissingPermission(reason) => {
                let msg = format!("You don't have permissions to do this: {}", reason);
                let template = ViewError::new("Error 403", 403, &msg);
                let body = template.render().unwrap();
                HttpResponse::NotFound().content_type("text/html").body(body)
            }
            e => {
                error!("Internal server error: {:#?}", e);
                let template = ViewError::new("Internal Server Error", 500, "Internal Server Error.");
                let body = template.render().unwrap();
                HttpResponse::InternalServerError().content_type("text/html").body(body)
            }
        }
    }
}

pub type AppResponse<T = HttpResponse> = Result<T, AppError>;

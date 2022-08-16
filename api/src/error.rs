use actix_web::{http::StatusCode, HttpResponse, ResponseError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Custom(String),
    #[error("{0}")]
    CustomStatus(StatusCode, String),
    #[error("{0}")]
    Unauthorized(String),
    #[error("Internal Server Error")]
    SessionInsertFailure(#[from] actix_session::SessionInsertError),
    #[error("Internal Server Error")]
    SessionGetFailure(#[from] actix_session::SessionGetError),
    #[error("You are not authorized to perform this action")]
    Forbidden,
    #[error("{0}")]
    Anyhow(#[from] anyhow::Error),
    #[error("Internal Connection Error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Database Error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
}

pub fn create_error(msg: &str) -> Error {
    Error::Custom(msg.to_string())
}

impl Error {
    pub fn new(status: StatusCode, msg: &str) -> Error {
        Error::CustomStatus(status, msg.to_string())
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Error::Reqwest(e) => e.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            Error::CustomStatus(code, _) => *code,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let code = self.status_code();
        let message = self.to_string();
        HttpResponse::build(code).body(message)
    }
}

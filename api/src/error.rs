use argonautica::Error as ArgonauticaError;
use actix_web::error::Error as ActixWebError;
use diesel::result::{Error as DieselError, DatabaseErrorKind};
use serde_json::json;
use std::{convert, error, fmt};

use actix_web::{error::ResponseError, HttpResponse};

#[derive(Debug)]
pub enum ApiError {
    InternalServerError(String),
    BadRequest(String),
    Unauthorized,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ApiError::*;

        let text = match self {
            InternalServerError(msg) => format!("Internal Server Error: {}", msg),
            BadRequest(msg) => format!("Bad Request: {}", msg),
            Unauthorized => "Unauthorized".to_string(),
        };

        write!(f, "{}", text)
    }
}

impl error::Error for ApiError {
    fn description(&self) -> &str {
        "API Error"
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl convert::From<ArgonauticaError> for ApiError {
    fn from(e: ArgonauticaError) -> ApiError {
        ApiError::InternalServerError(format!("{}", e))
    }
}

impl convert::From<ActixWebError> for ApiError {
    fn from(e: ActixWebError) -> ApiError {
        ApiError::InternalServerError(format!("{}", e))
    }
}

impl convert::From<DieselError> for ApiError {
    fn from(e: DieselError) -> ApiError {
        use DatabaseErrorKind::*;

        match e {
            DieselError::DatabaseError(kind, info) => {
                let message = info.details().unwrap_or_else(|| info.message()).to_string();

                match kind {
                    UniqueViolation | ForeignKeyViolation => ApiError::BadRequest(message),
                    _  => ApiError::InternalServerError(message),
                }
            }
            _ => ApiError::InternalServerError(format!("{}", e)),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        use ApiError::*;

        match self {
            InternalServerError(msg) =>
                HttpResponse::InternalServerError().json(json!({"message": msg.clone()})),
            BadRequest(msg) =>
                HttpResponse::BadRequest().json(json!({"message": msg.clone()})),
            Unauthorized =>
                HttpResponse::Unauthorized().finish(),
        }
    }
}

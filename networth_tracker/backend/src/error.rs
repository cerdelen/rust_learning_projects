use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("database error")]
	Db,
	#[error("wtfff")]
	Other(String)
}

impl ResponseError for Error {
	fn error_response(&self) -> HttpResponse {
		match self {
			Error::Db => HttpResponse::InternalServerError().body(self.to_string()),
			Error::Other(message) => HttpResponse::InternalServerError().body(message.clone()),
		}
	}
}

impl From<surrealdb::Error> for Error {
	fn from(error: surrealdb::Error) -> Self {
		eprintln!("{error}");
		Self::Db
	}
}
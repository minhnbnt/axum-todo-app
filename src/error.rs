use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use std::error::Error;

#[derive(Debug)]
pub struct AppError(Box<dyn Error>);

impl IntoResponse for AppError {
	fn into_response(self) -> Response {
		(
			StatusCode::INTERNAL_SERVER_ERROR,
			format!("Something went wrong: {}", self.0),
		)
			.into_response()
	}
}

impl<E> From<E> for AppError
where
	E: 'static + Error,
{
	fn from(err: E) -> Self {
		Self(Box::new(err))
	}
}

pub type AppResult<T> = Result<T, AppError>;

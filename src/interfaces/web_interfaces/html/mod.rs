pub mod html_formatter;
pub mod html_handlers;
pub mod web_routes;

use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;
use web_routes::WebRoutes;

use crate::{interfaces::lexicon::Lexicon, use_cases::VotingController};

#[derive(Error, Debug)]
#[error("Error: {0}")]
pub struct AxumError(#[from] anyhow::Error);

impl IntoResponse for AxumError {
	fn into_response(self) -> axum::response::Response {
		(
			StatusCode::INTERNAL_SERVER_ERROR,
			format!("Something went wrong: {}", self),
		)
			.into_response()
	}
}

#[derive(Clone)]
pub struct AxumState<Store> {
	pub controller: VotingController<Store>,
	pub routes: WebRoutes,
	pub lexicon: Lexicon,
}

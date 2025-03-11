pub mod html;
pub mod router;

use axum::{
	body::Body,
	http::{Response, StatusCode},
	response::IntoResponse,
};
use html::web_routes::WebRoutes;
use thiserror::Error;

use crate::use_cases::VotingController;

use super::lexicon::Lexicon;

#[derive(Error, Debug)]
#[error("Error: {0}")]
pub struct AxumError(#[from] anyhow::Error);

impl IntoResponse for AxumError {
	fn into_response(self) -> Response<Body> {
		(
			StatusCode::INTERNAL_SERVER_ERROR,
			format!("Something went wrong : {self}"),
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

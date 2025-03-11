use axum::{extract::State, response::IntoResponse, Form};

use crate::{
	domain::vote_outcome,
	interfaces::{
		show_vote_outcome,
		web_interfaces::{AxumError, AxumState},
	},
	storage::Storage,
	use_cases::VoteForm,
};

use super::html_formatter::{index, vote_form, voting_machine};

pub async fn get_index<Store: Storage>(
	State(app_state): State<AxumState<Store>>,
) -> Result<impl IntoResponse, AxumError> {
	Ok(index(
		&app_state.routes,
		&app_state.lexicon,
		&app_state.controller.get_voting_machine().await?,
	))
}

pub async fn get_results<Store: Storage>(
	State(app_state): State<AxumState<Store>>,
) -> Result<impl IntoResponse, AxumError> {
	Ok(voting_machine(
		&app_state.routes,
		&app_state.lexicon,
		&app_state.controller.get_voting_machine().await?,
	))
}

pub async fn vote<Store: Storage>(
	State(app_state): State<AxumState<Store>>,
	Form(vote_form): Form<VoteForm>,
) -> Result<impl IntoResponse, AxumError> {
	Ok(show_vote_outcome(
		app_state.controller.vote(vote_form).await?,
		&app_state.lexicon,
	))
}

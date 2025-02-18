#[derive(PartialEq, Eq, Clone)]
pub struct Lexicon {
	pub blank_vote: &'static str,
	pub has_already_voted: &'static str,
	pub accepted_vote: &'static str,
	pub invalid_vote: &'static str,
	pub vote: &'static str,
	pub blank: &'static str,
	pub invalid: &'static str,
	pub voters: &'static str,
	pub voters_title: &'static str,
	pub scores: &'static str,
	pub scores_title: &'static str,
	pub invalid_command: &'static str,
	pub help: &'static str,
	pub candidate_missing: &'static str,
}

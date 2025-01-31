use super::generic_domains::{Candidate, Voter};

#[derive(Clone, Debug)]
pub struct BallotPaper {
	pub voter: Voter,
	pub candidate: Option<Candidate>,
}

impl BallotPaper {
	#[must_use]
	pub const fn new(voter: Voter, candidate: Option<Candidate>) -> Self {
		Self { voter, candidate }
	}
}

use super::generic_domains::{Candidate, Voter};

pub struct BallotPaper {
	pub voter: Voter,
	pub candidate: Option<Candidate>,
}

impl BallotPaper {
	pub fn new(voter: Voter, candidate: Option<Candidate>) -> Self {
		Self { voter, candidate }
	}
}

use super::generic_domains::{Candidate, Voter};

#[derive(Debug)]
pub enum VoteOutcome {
	AcceptedVote(Voter, Candidate),
	BlankVote(Voter),
	InvalidVote(Voter),
	HasAlreadyVoted(Voter),
}

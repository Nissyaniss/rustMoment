use super::generic_domains::{Candidate, Voter};

pub enum VoteOutcome {
	AcceptedVote(Voter, Candidate),
	BlankVote(Voter),
	InvalidVote(Voter),
	HasAlreadyVoted(Voter),
}

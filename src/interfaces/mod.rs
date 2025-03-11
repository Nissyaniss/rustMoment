use lexicon::Lexicon;

use crate::domain::vote_outcome::VoteOutcome;

pub mod cli_interfaces;
pub mod lexicon;
pub mod lexicons;
pub mod web_interfaces;

fn show_vote_outcome(outcome: VoteOutcome, lexicon: &Lexicon) -> String {
	match outcome {
		VoteOutcome::AcceptedVote(voter, candidate) => {
			format!("{voter} {} {candidate}.", lexicon.accepted_vote)
		}
		VoteOutcome::BlankVote(voter) => format!("{voter} {}", lexicon.blank_vote),
		VoteOutcome::InvalidVote(voter) => format!("{voter} {}", lexicon.invalid_vote),
		VoteOutcome::HasAlreadyVoted(voter) => format!("{voter} {}", lexicon.has_already_voted),
	}
}

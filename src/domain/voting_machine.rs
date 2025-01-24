use super::{
	ballot_paper::{self, BallotPaper},
	generic_domains::AttendenceSheet,
	scoreboard::Scoreboard,
	vote_outcome::VoteOutcome,
};

pub struct VotingMachine {
	voters: AttendenceSheet,
	scoreboard: Scoreboard,
}

impl VotingMachine {
	pub const fn new(voters: AttendenceSheet, scoreboard: Scoreboard) -> Self {
		Self { voters, scoreboard }
	}

	pub fn vote(&mut self, ballot_paper: BallotPaper) -> VoteOutcome {
		let voter = ballot_paper.voter;
		let Some(candidate) = ballot_paper.candidate else {
			self.scoreboard.blank_score.0 += 1;
			self.voters.0.insert(voter.clone());
			return VoteOutcome::BlankVote(voter);
		};
		if self.voters.0.contains(&voter) {
			VoteOutcome::HasAlreadyVoted(voter)
		} else if let Some(score) = self.scoreboard.scores.get_mut(&candidate) {
			score.0 += 1;
			self.voters.0.insert(voter.clone());
			VoteOutcome::AcceptedVote(voter, candidate)
		} else {
			self.scoreboard.invalid_score.0 += 1;
			self.voters.0.insert(voter.clone());
			VoteOutcome::InvalidVote(voter)
		}
	}
}

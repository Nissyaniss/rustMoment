use super::{
	ballot_paper::BallotPaper, generic_domains::AttendenceSheet, scoreboard::Scoreboard,
	vote_outcome::VoteOutcome,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VotingMachine {
	voters: AttendenceSheet,
	scoreboard: Scoreboard,
}

impl VotingMachine {
	#[must_use]
	pub const fn new(voters: AttendenceSheet, scoreboard: Scoreboard) -> Self {
		Self { voters, scoreboard }
	}

	pub fn vote(&mut self, ballot_paper: BallotPaper) -> VoteOutcome {
		let voter = ballot_paper.voter;
		if !self.voters.0.contains(&voter) {
			let Some(candidate) = ballot_paper.candidate else {
				self.scoreboard.blank_score.0 += 1;
				self.voters.0.insert(voter.clone());
				println!("{voter} a voter blanc!");
				return VoteOutcome::BlankVote(voter);
			};
			if let Some(score) = self.scoreboard.scores.get_mut(&candidate) {
				score.0 += 1;
				self.voters.0.insert(voter.clone());
				println!("{voter} a voter {candidate}!");
				return VoteOutcome::AcceptedVote(voter, candidate);
			} else {
				self.scoreboard.invalid_score.0 += 1;
				self.voters.0.insert(voter.clone());
				println!("{voter} a voter nul!");
				return VoteOutcome::InvalidVote(voter);
			}
		}
		println!("{voter} a deja voter!");
		VoteOutcome::HasAlreadyVoted(voter)
	}

	#[must_use]
	pub const fn get_scoreboard(&self) -> &Scoreboard {
		&self.scoreboard
	}

	#[must_use]
	pub const fn get_voter(&self) -> &AttendenceSheet {
		&self.voters
	}
}

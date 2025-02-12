use serde::Deserialize;

use crate::{
	domain::{
		ballot_paper::BallotPaper,
		generic_domains::{Candidate, Voter},
		vote_outcome::VoteOutcome,
		voting_machine::VotingMachine,
	},
	storage::Storage,
};

#[derive(Deserialize, Debug)]
pub struct VoteForm {
	pub voter: String,
	pub candidate: String,
}

impl From<BallotPaper> for VoteForm {
	fn from(value: BallotPaper) -> Self {
		Self {
			voter: value.voter.0,
			candidate: {
				match value.candidate {
					Some(value) => value.0,
					None => String::default(),
				}
			},
		}
	}
}

impl From<VoteForm> for BallotPaper {
	fn from(value: VoteForm) -> Self {
		if value.candidate.is_empty() {
			Self {
				voter: Voter(value.voter),
				candidate: None,
			}
		} else {
			Self {
				voter: Voter(value.voter),
				candidate: Some(Candidate(value.candidate)),
			}
		}
	}
}

#[derive(Debug)]
pub struct VotingController<Store> {
	store: Store,
}

impl<Store: Storage> VotingController<Store> {
	pub const fn new(store: Store) -> Self {
		Self { store }
	}

	pub async fn vote(&mut self, vote_forme: VoteForm) -> anyhow::Result<VoteOutcome> {
		let mut voting_machine = self.store.get_voting_machine().await?;
		let outcome = voting_machine.vote(vote_forme.into());
		self.store.put_voting_machine(voting_machine).await?;
		Ok(outcome)
	}

	pub async fn get_voting_machine(&self) -> anyhow::Result<VotingMachine> {
		self.store.get_voting_machine().await
	}
}

#[cfg(test)]
mod tests {
	use std::{collections::BTreeMap, process::exit};

	use crate::{
		domain::{
			ballot_paper::BallotPaper,
			generic_domains::{AttendenceSheet, Candidate, Score, Voter},
			scoreboard::Scoreboard,
			vote_outcome::VoteOutcome,
			voting_machine::VotingMachine,
		},
		storage::Storage,
		storages::memory::MemoryStore,
		use_cases::VotingController,
	};

	#[tokio::test]
	async fn normal_vote() {
		let mut tableau_candidats = BTreeMap::new();
		let candidate = Candidate("Macron".to_string());
		tableau_candidats.insert(candidate.clone(), Score(0));

		let scoreboard = Scoreboard {
			scores: tableau_candidats,
			blank_score: Score::default(),
			invalid_score: Score::default(),
		};

		let voters = AttendenceSheet::default();

		let voting_machine = VotingMachine::new(voters, scoreboard);

		let memory = match MemoryStore::new(voting_machine).await {
			Ok(memory) => memory,
			Err(e) => {
				println!("error : {e}");
				exit(1)
			}
		};
		let mut controller = VotingController::new(memory);
		let voter = Voter("Malo".to_string());
		let ballot_paper = BallotPaper::new(voter.clone(), Some(candidate.clone()));
		let outcome = match controller.vote(ballot_paper.into()).await {
			Ok(outcome) => outcome,
			Err(e) => {
				println!("error : {e}");
				exit(1)
			}
		};

		let mut correct_voters = AttendenceSheet::default();
		let mut correct_scores = BTreeMap::new();
		correct_scores.insert(candidate.clone(), Score(1));
		let correct_scoreboard = Scoreboard {
			scores: correct_scores,
			blank_score: Score(0),
			invalid_score: Score(0),
		};
		correct_voters.0.insert(voter.clone());
		let voting_machine = match controller.get_voting_machine().await {
			Ok(voting_machine) => voting_machine,
			Err(e) => {
				println!("error : {e}");
				exit(1)
			}
		};
		let correct_outcome = VoteOutcome::AcceptedVote(voter, candidate);
		assert_eq!(correct_outcome, outcome);
		assert_eq!(&correct_scoreboard, voting_machine.get_scoreboard());
	}

	#[tokio::test]
	async fn already_voted() {
		let mut tableau_candidats = BTreeMap::new();
		let candidate = Candidate("Macron".to_string());
		let voter = Voter("Malo".to_string());
		tableau_candidats.insert(candidate.clone(), Score(1));

		let scoreboard = Scoreboard {
			scores: tableau_candidats,
			blank_score: Score::default(),
			invalid_score: Score::default(),
		};

		let mut voters = AttendenceSheet::default();
		voters.0.insert(voter.clone());

		let voting_machine = VotingMachine::new(voters, scoreboard);

		let memory = match MemoryStore::new(voting_machine).await {
			Ok(memory) => memory,
			Err(e) => {
				println!("error : {e}");
				exit(1)
			}
		};
		let mut controller = VotingController::new(memory);
		let ballot_paper = BallotPaper::new(voter.clone(), Some(candidate.clone()));
		let outcome = match controller.vote(ballot_paper.into()).await {
			Ok(outcome) => outcome,
			Err(e) => {
				println!("error : {e}");
				exit(1)
			}
		};

		let mut correct_voters = AttendenceSheet::default();
		let mut correct_scores = BTreeMap::new();
		correct_scores.insert(candidate.clone(), Score(1));
		let correct_scoreboard = Scoreboard {
			scores: correct_scores,
			blank_score: Score(0),
			invalid_score: Score(0),
		};
		correct_voters.0.insert(voter.clone());
		let voting_machine = match controller.get_voting_machine().await {
			Ok(voting_machine) => voting_machine,
			Err(e) => {
				println!("error : {e}");
				exit(1)
			}
		};
		let correct_outcome = VoteOutcome::HasAlreadyVoted(voter);
		assert_eq!(correct_outcome, outcome);
		assert_eq!(&correct_scoreboard, voting_machine.get_scoreboard());
	}

	#[tokio::test]
	async fn vote_invalid() {
		let tableau_candidats = BTreeMap::new();
		let candidate = Candidate("Macron".to_string());
		let voter = Voter("Malo".to_string());

		let scoreboard = Scoreboard {
			scores: tableau_candidats,
			blank_score: Score::default(),
			invalid_score: Score::default(),
		};

		let voters = AttendenceSheet::default();

		let voting_machine = VotingMachine::new(voters, scoreboard);

		let memory = match MemoryStore::new(voting_machine).await {
			Ok(memory) => memory,
			Err(e) => {
				println!("error : {e}");
				exit(1)
			}
		};
		let mut controller = VotingController::new(memory);
		let ballot_paper = BallotPaper::new(voter.clone(), Some(candidate.clone()));
		let outcome = match controller.vote(ballot_paper.into()).await {
			Ok(outcome) => outcome,
			Err(e) => {
				println!("error : {e}");
				exit(1)
			}
		};

		let mut correct_voters = AttendenceSheet::default();
		let correct_scores = BTreeMap::new();
		let correct_scoreboard = Scoreboard {
			scores: correct_scores,
			blank_score: Score(0),
			invalid_score: Score(1),
		};
		correct_voters.0.insert(voter.clone());
		let voting_machine = match controller.get_voting_machine().await {
			Ok(voting_machine) => voting_machine,
			Err(e) => {
				println!("error : {e}");
				exit(1)
			}
		};
		let correct_outcome = VoteOutcome::InvalidVote(voter);
		assert_eq!(correct_outcome, outcome);
		assert_eq!(&correct_scoreboard, voting_machine.get_scoreboard());
	}

	#[tokio::test]
	async fn vote_no_one() {
		let mut tableau_candidats = BTreeMap::new();
		let candidate = Candidate("Macron".to_string());
		let voter = Voter("Malo".to_string());
		tableau_candidats.insert(candidate.clone(), Score(0));

		let scoreboard = Scoreboard {
			scores: tableau_candidats,
			blank_score: Score::default(),
			invalid_score: Score::default(),
		};

		let voters = AttendenceSheet::default();

		let voting_machine = VotingMachine::new(voters, scoreboard);

		let memory = match MemoryStore::new(voting_machine).await {
			Ok(memory) => memory,
			Err(e) => {
				println!("error : {e}");
				exit(1)
			}
		};
		let mut controller = VotingController::new(memory);
		let ballot_paper = BallotPaper::new(voter.clone(), None);
		let outcome = match controller.vote(ballot_paper.into()).await {
			Ok(outcome) => outcome,
			Err(e) => {
				println!("error : {e}");
				exit(1)
			}
		};

		let mut correct_voters = AttendenceSheet::default();
		let mut correct_scores = BTreeMap::new();
		correct_scores.insert(candidate.clone(), Score(0));
		let correct_scoreboard = Scoreboard {
			scores: correct_scores,
			blank_score: Score(1),
			invalid_score: Score(0),
		};
		correct_voters.0.insert(voter.clone());
		let voting_machine = match controller.get_voting_machine().await {
			Ok(voting_machine) => voting_machine,
			Err(e) => {
				println!("error : {e}");
				exit(1)
			}
		};
		let correct_outcome = VoteOutcome::BlankVote(voter);
		assert_eq!(correct_outcome, outcome);
		assert_eq!(&correct_scoreboard, voting_machine.get_scoreboard());
	}
}

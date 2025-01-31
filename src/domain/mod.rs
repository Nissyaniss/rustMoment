pub mod ballot_paper;
pub mod generic_domains;
pub mod scoreboard;
pub mod vote_outcome;
pub mod voting_machine;

#[cfg(test)]
mod tests {
	use std::collections::BTreeMap;

	use super::{
		ballot_paper::BallotPaper,
		generic_domains::{AttendenceSheet, Candidate, Score, Voter},
		scoreboard::Scoreboard,
		voting_machine::VotingMachine,
	};

	#[test]
	fn normal_vote() {
		let voter = Voter("Malo".to_string());
		let candidate = Candidate("Philipe_Poutou".to_string());

		let mut tableau_candidats = BTreeMap::new();

		tableau_candidats.insert(candidate.clone(), Score::default());

		let scoreboard = Scoreboard {
			scores: tableau_candidats,
			blank_score: Score::default(),
			invalid_score: Score::default(),
		};

		let voters = AttendenceSheet::default();

		let mut voting_machine = VotingMachine::new(voters, scoreboard);

		let ballot_paper = BallotPaper {
			voter: voter.clone(),
			candidate: Some(candidate.clone()),
		};
		voting_machine.vote(ballot_paper);

		let mut correct_voters = AttendenceSheet::default();
		let mut correct_scores = BTreeMap::new();
		correct_scores.insert(candidate, Score(1));
		let correct_scoreboard = Scoreboard {
			scores: correct_scores,
			blank_score: Score(0),
			invalid_score: Score(0),
		};
		correct_voters.0.insert(voter);
		assert_eq!(&correct_voters, voting_machine.get_voter());
		assert_eq!(&correct_scoreboard, voting_machine.get_scoreboard());
	}

	#[test]
	fn alredy_voted() {
		let voter = Voter("Malo".to_string());
		let candidate = Candidate("Philipe_Poutou".to_string());

		let mut tableau_candidats = BTreeMap::new();

		tableau_candidats.insert(candidate.clone(), Score::default());

		let scoreboard = Scoreboard {
			scores: tableau_candidats,
			blank_score: Score::default(),
			invalid_score: Score::default(),
		};

		let voters = AttendenceSheet::default();

		let mut voting_machine = VotingMachine::new(voters, scoreboard);

		let ballot_paper = BallotPaper {
			voter: voter.clone(),
			candidate: Some(candidate.clone()),
		};
		voting_machine.vote(ballot_paper.clone());
		voting_machine.vote(ballot_paper);

		let mut correct_voters = AttendenceSheet::default();
		correct_voters.0.insert(voter);
		let mut correct_scores = BTreeMap::new();
		correct_scores.insert(candidate, Score(1));
		let correct_scoreboard = Scoreboard {
			scores: correct_scores,
			blank_score: Score(0),
			invalid_score: Score(0),
		};
		assert_eq!(&correct_voters, voting_machine.get_voter());
		assert_eq!(&correct_scoreboard, voting_machine.get_scoreboard());
	}

	#[test]
	fn vote_invalid() {
		let voter = Voter("Malo".to_string());
		let candidate = Candidate("Philipe_Poutou".to_string());

		let scoreboard = Scoreboard {
			scores: BTreeMap::new(),
			blank_score: Score::default(),
			invalid_score: Score::default(),
		};

		let voters = AttendenceSheet::default();

		let mut voting_machine = VotingMachine::new(voters, scoreboard);

		let ballot_paper = BallotPaper {
			voter: voter.clone(),
			candidate: Some(candidate),
		};
		voting_machine.vote(ballot_paper);

		let mut correct_voters = AttendenceSheet::default();
		correct_voters.0.insert(voter);
		let correct_scoreboard = Scoreboard {
			scores: BTreeMap::new(),
			blank_score: Score(0),
			invalid_score: Score(1),
		};
		assert_eq!(&correct_voters, voting_machine.get_voter());
		assert_eq!(&correct_scoreboard, voting_machine.get_scoreboard());
	}

	#[test]
	fn vote_no_one() {
		let voter = Voter("Malo".to_string());
		let scoreboard = Scoreboard {
			scores: BTreeMap::new(),
			blank_score: Score::default(),
			invalid_score: Score::default(),
		};

		let voters = AttendenceSheet::default();

		let mut voting_machine = VotingMachine::new(voters, scoreboard);

		let ballot_paper = BallotPaper {
			voter: voter.clone(),
			candidate: None,
		};
		voting_machine.vote(ballot_paper);

		let mut correct_voters = AttendenceSheet::default();
		correct_voters.0.insert(voter);
		let correct_scoreboard = Scoreboard {
			scores: BTreeMap::new(),
			blank_score: Score(1),
			invalid_score: Score(0),
		};
		assert_eq!(&correct_voters, voting_machine.get_voter());
		assert_eq!(&correct_scoreboard, voting_machine.get_scoreboard());
	}
}

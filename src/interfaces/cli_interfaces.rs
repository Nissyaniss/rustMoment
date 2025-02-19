use super::lexicon::Lexicon;
use crate::domain::ballot_paper::BallotPaper;
use crate::domain::generic_domains::AttendenceSheet;
use crate::domain::generic_domains::Candidate;
use crate::domain::generic_domains::Voter;
use crate::domain::scoreboard::Scoreboard;
use crate::domain::vote_outcome::VoteOutcome;
use crate::{storage::Storage, use_cases::VotingController};

fn show_attendence_sheet(voters: &AttendenceSheet, lexicon: &Lexicon) -> String {
	let mut res = lexicon.voters_title.to_string();
	for voter in voters.0.clone() {
		res += &format!("- {voter}\n");
	}
	res
}

fn show_scoreboard(scores: &Scoreboard, lexicon: &Lexicon) -> String {
	let mut res = lexicon.scores_title.to_string();
	for (nom, score) in scores.scores.clone() {
		res += &format!("{nom}: {score}\n");
	}
	res += &format!("{}: {}\n", lexicon.blank, scores.blank_score);
	res += &format!("{}: {}", lexicon.invalid, scores.invalid_score);
	res
}

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

pub async fn handle_line<Store: Storage>(
	line: &str,
	controller: VotingController<Store>,
	lexicon: &Lexicon,
) -> anyhow::Result<String> {
	let voting_machine = controller.get_voting_machine().await?;
	let mut mots = line.split(' ');
	let premier_mot = mots.next().unwrap_or_default();
	let deuxieme_mot = mots.next().unwrap_or_default();
	let troisieme_mot = mots.next().unwrap_or_default();
	let res = if premier_mot == lexicon.vote {
		if !deuxieme_mot.is_empty() && !troisieme_mot.is_empty() {
			let ballot_paper = BallotPaper {
				voter: Voter(deuxieme_mot.to_string()),
				candidate: Some(Candidate(troisieme_mot.to_string())),
			};
			show_vote_outcome(controller.clone().vote(ballot_paper.into()).await?, lexicon)
		} else if !deuxieme_mot.is_empty() {
			let ballot_paper = BallotPaper {
				voter: Voter(deuxieme_mot.to_string()),
				candidate: None,
			};
			show_vote_outcome(controller.clone().vote(ballot_paper.into()).await?, lexicon)
		} else {
			lexicon.candidate_missing.to_string()
		}
	} else if premier_mot == lexicon.voters {
		show_attendence_sheet(voting_machine.get_voter(), lexicon)
	} else if premier_mot == lexicon.scores {
		show_scoreboard(voting_machine.get_scoreboard(), lexicon)
	} else if line.is_empty() {
		lexicon.help.to_string()
	} else {
		lexicon.invalid_command.to_string()
	};
	Ok(res)
}

#[cfg(test)]
mod tests {
	use std::collections::BTreeMap;

	use crate::{
		domain::{
			generic_domains::{AttendenceSheet, Candidate, Score},
			scoreboard::Scoreboard,
			voting_machine::VotingMachine,
		},
		interfaces::{cli_interfaces::handle_line, lexicon::Lexicon},
		storage::Storage,
		storages::memory::MemoryStore,
		use_cases::VotingController,
	};

	#[tokio::test]
	async fn no_command() {
		let lexicon = Lexicon::french();
		let mut tableau_candidats = BTreeMap::new();
		let candidate = Candidate("Yay".to_string());
		tableau_candidats.insert(candidate, Score::default());

		let scoreboard = Scoreboard {
			scores: tableau_candidats,
			blank_score: Score::default(),
			invalid_score: Score::default(),
		};

		let voters = AttendenceSheet::default();

		let voting_machine = VotingMachine::new(voters, scoreboard);

		let memory = MemoryStore::new(voting_machine).await.unwrap();
		let mut controller = VotingController::new(memory);

		assert_eq!(
			"Aide :\n - voter <nom> [candidat]\n - scores\n - votants".to_string(),
			handle_line("", controller, &lexicon).await.unwrap()
		);
	}

	#[tokio::test]
	async fn show_voters() {
		let lexicon = Lexicon::french();
		let mut tableau_candidats = BTreeMap::new();
		let candidate = Candidate("Yay".to_string());
		tableau_candidats.insert(candidate, Score::default());

		let scoreboard = Scoreboard {
			scores: tableau_candidats,
			blank_score: Score::default(),
			invalid_score: Score::default(),
		};

		let voters = AttendenceSheet::default();

		let voting_machine = VotingMachine::new(voters, scoreboard);

		let memory = MemoryStore::new(voting_machine).await.unwrap();
		let mut controller = VotingController::new(memory);

		assert_eq!(
			"Voici les votants:\n".to_string(),
			handle_line("votants", controller, &lexicon).await.unwrap()
		);
	}

	#[tokio::test]
	async fn show_scores() {
		let lexicon = Lexicon::french();
		let mut tableau_candidats = BTreeMap::new();
		let candidate = Candidate("Yay".to_string());
		tableau_candidats.insert(candidate, Score::default());

		let scoreboard = Scoreboard {
			scores: tableau_candidats,
			blank_score: Score::default(),
			invalid_score: Score::default(),
		};

		let voters = AttendenceSheet::default();

		let voting_machine = VotingMachine::new(voters, scoreboard);

		let memory = MemoryStore::new(voting_machine).await.unwrap();
		let mut controller = VotingController::new(memory);

		assert_eq!(
			"Voici les scores:\nYay: 0\nBlanc: 0\nNull: 0".to_string(),
			handle_line("scores", controller, &lexicon).await.unwrap()
		);
	}

	#[tokio::test]
	async fn vote() {
		let lexicon = Lexicon::french();
		let mut tableau_candidats = BTreeMap::new();
		let candidate = Candidate("MacOS".to_string());
		tableau_candidats.insert(candidate, Score::default());

		let scoreboard = Scoreboard {
			scores: tableau_candidats,
			blank_score: Score::default(),
			invalid_score: Score::default(),
		};

		let voters = AttendenceSheet::default();

		let voting_machine = VotingMachine::new(voters, scoreboard);

		let memory = MemoryStore::new(voting_machine).await.unwrap();
		let mut controller = VotingController::new(memory);

		assert_eq!(
			"moi a voter pour MacOS.".to_string(),
			handle_line("voter moi MacOS", controller, &lexicon)
				.await
				.unwrap()
		);
	}

	#[tokio::test]
	async fn vote_blank() {
		let lexicon = Lexicon::french();
		let mut tableau_candidats = BTreeMap::new();
		let candidate = Candidate("Yay".to_string());
		tableau_candidats.insert(candidate, Score::default());

		let scoreboard = Scoreboard {
			scores: tableau_candidats,
			blank_score: Score::default(),
			invalid_score: Score::default(),
		};

		let voters = AttendenceSheet::default();

		let voting_machine = VotingMachine::new(voters, scoreboard);

		let memory = MemoryStore::new(voting_machine).await.unwrap();
		let mut controller = VotingController::new(memory);

		assert_eq!(
			"moi a voter blanc.".to_string(),
			handle_line("voter moi", controller, &lexicon)
				.await
				.unwrap()
		);
	}

	#[tokio::test]
	async fn missing_voter() {
		let lexicon = Lexicon::french();
		let mut tableau_candidats = BTreeMap::new();
		let candidate = Candidate("Yay".to_string());
		tableau_candidats.insert(candidate, Score::default());

		let scoreboard = Scoreboard {
			scores: tableau_candidats,
			blank_score: Score::default(),
			invalid_score: Score::default(),
		};

		let voters = AttendenceSheet::default();

		let voting_machine = VotingMachine::new(voters, scoreboard);

		let memory = MemoryStore::new(voting_machine).await.unwrap();
		let mut controller = VotingController::new(memory);

		assert_eq!(
			"Il manque un votant.".to_string(),
			handle_line("voter", controller, &lexicon).await.unwrap()
		);
	}

	#[tokio::test]
	async fn invalid_command() {
		let lexicon = Lexicon::french();
		let mut tableau_candidats = BTreeMap::new();
		let candidate = Candidate("Yay".to_string());
		tableau_candidats.insert(candidate, Score::default());

		let scoreboard = Scoreboard {
			scores: tableau_candidats,
			blank_score: Score::default(),
			invalid_score: Score::default(),
		};

		let voters = AttendenceSheet::default();

		let voting_machine = VotingMachine::new(voters, scoreboard);

		let memory = MemoryStore::new(voting_machine).await.unwrap();
		let mut controller = VotingController::new(memory);

		assert_eq!(
			"Commande non valide".to_string(),
			handle_line("zxdfsdf", controller, &lexicon).await.unwrap()
		);
	}
}

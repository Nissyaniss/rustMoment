use std::collections::BTreeMap;

use crate::{
	configuration::Configuration,
	domain::{
		ballot_paper::BallotPaper,
		generic_domains::{AttendenceSheet, Candidate, Score, Voter},
		scoreboard::Scoreboard,
		voting_machine::VotingMachine,
	},
};
use tokio::io::{self, AsyncBufReadExt, BufReader};

pub async fn run_app(configuration: Configuration) -> anyhow::Result<()> {
	let mut tableau_candidats = BTreeMap::new();

	for candidates in configuration.candidates {
		let candidate = Candidate(candidates);
		tableau_candidats.insert(candidate, Score::default());
	}

	let scoreboard = Scoreboard {
		scores: tableau_candidats,
		blank_score: Score::default(),
		invalid_score: Score::default(),
	};

	let voters = AttendenceSheet::default();

	let mut voting_machine = VotingMachine::new(voters, scoreboard);

	loop {
		let mut lines = BufReader::new(io::stdin()).lines();
		if let Some(line) = lines.next_line().await? {
			let mut mots = line.split(' ');
			let premier_mot = mots.next().unwrap_or_default();
			let deuxieme_mot = mots.next().unwrap_or_default();
			let troisieme_mot = mots.next().unwrap_or_default();
			match premier_mot {
				"voter" => {
					if !deuxieme_mot.is_empty() && !troisieme_mot.is_empty() {
						let ballot_paper = BallotPaper {
							voter: Voter(deuxieme_mot.to_string()),
							candidate: Some(Candidate(troisieme_mot.to_string())),
						};
						voting_machine.vote(ballot_paper);
					} else if !deuxieme_mot.is_empty() {
						let ballot_paper = BallotPaper {
							voter: Voter(deuxieme_mot.to_string()),
							candidate: None,
						};
						voting_machine.vote(ballot_paper);
					} else {
						println!("Pas bien");
					}
				}
				"votants" => afficher_votants(voting_machine.get_voter()),
				"scores" => afficher_score(voting_machine.get_scoreboard()),
				_ => println!("Commande invalide"),
			}
		}
	}
}

fn afficher_score(scores: &Scoreboard) {
	println!("\nVoici les scores\n");
	for (nom, score) in scores.scores.clone() {
		println!("{nom}: {score}");
	}
	println!("Blanc: {}", scores.blank_score);
	println!("Nul: {}", scores.invalid_score);
}

fn afficher_votants(votants: &AttendenceSheet) {
	println!("\nVoici les votants\n");
	for votant in votants.0.clone() {
		println!("{votant}");
	}
}

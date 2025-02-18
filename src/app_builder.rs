use std::collections::BTreeMap;

use crate::{
	configuration::{Configuration, LanguageType, StoredType},
	domain::{
		generic_domains::{AttendenceSheet, Candidate, Score},
		scoreboard::Scoreboard,
		voting_machine::VotingMachine,
	},
	interfaces::{cli_interfaces::handle_line, lexicon::Lexicon},
	storage::Storage,
	storages::{file::FileStore, memory::MemoryStore},
	use_cases::VotingController,
};
use tokio::io::{self, AsyncBufReadExt, BufReader};

/// # Errors
///
/// Will return `Err` if `handle_lines` exits with an error
pub async fn run_app(configuration: Configuration) -> anyhow::Result<()> {
	match configuration.storage {
		StoredType::File => handle_lines::<FileStore>(configuration).await,
		StoredType::Memory => handle_lines::<MemoryStore>(configuration).await,
	}
}

/// # Errors
///
/// Will return `Err` if `handle_lines` exits with an error
pub async fn handle_lines<Store: Storage>(configuration: Configuration) -> anyhow::Result<()> {
	let mut tableau_candidats = BTreeMap::new();

	for candidates in configuration.candidates {
		let candidate = Candidate(candidates);
		tableau_candidats.insert(candidate, Score::default());
	}

	let lexicon = match configuration.language {
		LanguageType::Fr => Lexicon::french(),
		LanguageType::En => Lexicon::english(),
	};

	let scoreboard = Scoreboard {
		scores: tableau_candidats,
		blank_score: Score::default(),
		invalid_score: Score::default(),
	};

	let voters = AttendenceSheet::default();

	let voting_machine = VotingMachine::new(voters, scoreboard);

	let memory = Store::new(voting_machine).await?;
	let mut controller = VotingController::new(memory);

	loop {
		let mut lines = BufReader::new(io::stdin()).lines();
		if let Some(line) = lines.next_line().await? {
			println!("{}", handle_line(&line, &mut controller, &lexicon).await?);
		}
	}
}

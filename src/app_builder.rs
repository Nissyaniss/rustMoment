use std::collections::BTreeMap;

use crate::{
	configuration::{Configuration, LanguageType, StoredType},
	domain::{
		generic_domains::{AttendenceSheet, Candidate, Score},
		scoreboard::Scoreboard,
		voting_machine::VotingMachine,
	},
	interfaces::lexicon::Lexicon,
	service::Service,
	services::stdio::StdioService,
	storage::Storage,
	storages::{file::FileStore, memory::MemoryStore},
	use_cases::VotingController,
};

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
pub async fn handle_lines<Store: Storage + Send + Sync>(
	configuration: Configuration,
) -> anyhow::Result<()> {
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
	let controller = VotingController::new(memory);

	let mut stdio_service = StdioService::new(0, lexicon, controller);

	stdio_service.serve().await
}

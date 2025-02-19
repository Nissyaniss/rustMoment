use std::collections::BTreeMap;

use crate::{
	configuration::{Configuration, LanguageType, ServiceType, StoredType},
	domain::{
		generic_domains::{AttendenceSheet, Candidate, Score},
		scoreboard::Scoreboard,
		voting_machine::VotingMachine,
	},
	interfaces::lexicon::Lexicon,
	service::Service,
	services::{stdio::StdioService, tcp::TcpService, udp::UdpService},
	storage::Storage,
	storages::{file::FileStore, memory::MemoryStore},
	use_cases::VotingController,
};

/// # Errors
///
/// Will return `Err` if `handle_lines` exits with an error
pub async fn run_app(configuration: Configuration) -> anyhow::Result<()> {
	match configuration.storage {
		StoredType::File => dispatch_service::<FileStore>(configuration).await,
		StoredType::Memory => dispatch_service::<MemoryStore>(configuration).await,
	}
}

pub async fn dispatch_service<Store: Storage + Send + Sync + Clone + 'static>(
	configuration: Configuration,
) -> anyhow::Result<()> {
	match configuration.service {
		ServiceType::Stdio => handle_lines::<Store, StdioService<Store>>(configuration).await,
		ServiceType::Udp => handle_lines::<Store, UdpService<Store>>(configuration).await,
		ServiceType::Tcp => handle_lines::<Store, TcpService<Store>>(configuration).await,
	}
}

/// # Errors
///
/// Will return `Err` if `handle_line` exits with an error
pub async fn handle_lines<Store: Storage, Serv: Service<Store>>(
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

	Serv::new(configuration.port, lexicon, controller)
		.serve()
		.await
}

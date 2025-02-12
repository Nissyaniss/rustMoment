use std::{
	collections::{BTreeMap, BTreeSet},
	path::Path,
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::{
	fs::File,
	io::{AsyncReadExt, AsyncWriteExt},
};

use crate::{
	domain::{
		generic_domains::{AttendenceSheet, Candidate, Score, Voter},
		scoreboard::Scoreboard,
		voting_machine::VotingMachine,
	},
	storage::Storage,
};

pub struct FileStore {
	filepath: String,
}

const FILEPATH: &str = "machine.json";

impl FileStore {
	pub async fn create(machine: VotingMachine, filepath: &str) -> anyhow::Result<Self> {
		if !Path::new(filepath).exists() {
			let mut file = File::create(filepath).await?;
			file.write_all(serde_json::to_string(&VotingMachineDAO::from(machine))?.as_bytes())
				.await?;
		}
		Ok(Self {
			filepath: filepath.to_string(),
		})
	}
}

#[async_trait]
impl Storage for FileStore {
	async fn new(machine: VotingMachine) -> anyhow::Result<Self> {
		Self::create(machine, FILEPATH).await
	}

	async fn get_voting_machine(&self) -> anyhow::Result<VotingMachine> {
		let mut file = File::open(self.filepath.clone()).await?;
		let file_string = &mut Vec::new();
		file.read_to_end(file_string).await?;
		let mut deserializer = serde_json::Deserializer::from_slice(file_string);
		Ok(VotingMachine::from(VotingMachineDAO::deserialize(
			&mut deserializer,
		)?))
	}

	async fn put_voting_machine(&mut self, machine: VotingMachine) -> anyhow::Result<()> {
		let mut file = File::create(self.filepath.clone()).await?;
		let voting_machine_json = serde_json::to_string(&VotingMachineDAO::from(machine))?;
		file.write_all(voting_machine_json.as_bytes()).await?;
		Ok(())
	}
}

#[derive(Serialize, Deserialize)]
struct ScoreboardDAO {
	scores: BTreeMap<String, usize>,
	blank_scores: usize,
	invalid_scores: usize,
}

impl From<Scoreboard> for ScoreboardDAO {
	fn from(value: Scoreboard) -> Self {
		let mut scores = BTreeMap::new();
		for (candidate, score) in value.scores {
			scores.insert(candidate.0, score.0);
		}
		Self {
			scores,
			blank_scores: value.blank_score.0,
			invalid_scores: value.invalid_score.0,
		}
	}
}

impl From<ScoreboardDAO> for Scoreboard {
	fn from(value: ScoreboardDAO) -> Self {
		let mut scores = BTreeMap::new();
		for (candidate, score) in value.scores {
			scores.insert(Candidate(candidate), Score(score));
		}
		Self {
			scores,
			blank_score: Score(value.blank_scores),
			invalid_score: Score(value.invalid_scores),
		}
	}
}

#[derive(Serialize, Deserialize)]
struct VotingMachineDAO {
	voters: BTreeSet<String>,
	scoreboard: ScoreboardDAO,
}

impl From<VotingMachine> for VotingMachineDAO {
	fn from(value: VotingMachine) -> Self {
		let mut voters = BTreeSet::new();
		for voter in value.get_voter().clone().0 {
			voters.insert(voter.0);
		}

		let scoreboard = ScoreboardDAO::from(value.get_scoreboard().clone());
		Self { voters, scoreboard }
	}
}

impl From<VotingMachineDAO> for VotingMachine {
	fn from(value: VotingMachineDAO) -> Self {
		let mut voters = BTreeSet::new();
		for voter in value.voters {
			voters.insert(Voter(voter));
		}

		let attendence_sheet = AttendenceSheet(voters);

		let scoreboard = Scoreboard::from(value.scoreboard);
		Self::new(attendence_sheet, scoreboard)
	}
}

#[tokio::test]
async fn my_test() {
	let mut tableau_candidats = BTreeMap::new();

	tableau_candidats.insert(Candidate("moi".to_string()), Score::default());

	let scoreboard = Scoreboard {
		scores: tableau_candidats,
		blank_score: Score::default(),
		invalid_score: Score::default(),
	};

	let voters = AttendenceSheet::default();

	let voting_machine = VotingMachine::new(voters, scoreboard);

	let store = FileStore::create(voting_machine.clone(), "test.json").await;

	let stored_machine = store.unwrap().get_voting_machine().await.unwrap();

	assert_eq!(voting_machine, stored_machine);
}

#[tokio::test]
async fn my_test2() {
	let mut tableau_candidats = BTreeMap::new();

	tableau_candidats.insert(Candidate("moi".to_string()), Score::default());

	let scoreboard = Scoreboard {
		scores: tableau_candidats,
		blank_score: Score::default(),
		invalid_score: Score::default(),
	};

	let voters = AttendenceSheet::default();

	let voting_machine = VotingMachine::new(voters, scoreboard);

	let store = FileStore::create(voting_machine.clone(), "test.json").await;
	let store2 = FileStore::create(voting_machine.clone(), "test.json").await;

	let stored_machine = store.unwrap().get_voting_machine().await.unwrap();
	let stored_machine2 = store2.unwrap().get_voting_machine().await.unwrap();

	assert_eq!(stored_machine, stored_machine2);
}

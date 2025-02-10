pub mod file;
pub mod memory;

#[tokio::test]
async fn my_test() {
	use memory::MemoryStore;
	use std::collections::BTreeMap;

	use crate::domain::{
		generic_domains::{AttendenceSheet, Candidate, Score},
		scoreboard::Scoreboard,
		voting_machine::VotingMachine,
	};
	use crate::storage::Storage;

	let mut tableau_candidats = BTreeMap::new();

	tableau_candidats.insert(Candidate("moi".to_string()), Score::default());

	let scoreboard = Scoreboard {
		scores: tableau_candidats,
		blank_score: Score::default(),
		invalid_score: Score::default(),
	};

	let voters = AttendenceSheet::default();

	let voting_machine = VotingMachine::new(voters, scoreboard);

	let store = MemoryStore::new(voting_machine.clone()).await;

	let stored_machine = store.unwrap().get_voting_machine().await.unwrap();

	assert_eq!(voting_machine, stored_machine);
}

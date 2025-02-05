use std::collections::BTreeMap;

use async_trait::async_trait;

use crate::{domain::{generic_domains::{AttendenceSheet, Candidate, Score}, scoreboard::Scoreboard, voting_machine::VotingMachine}, storage::Storage};

struct MemoryStore {
	voting_machine: VotingMachine,
}

#[async_trait]
impl Storage for MemoryStore {
    
    async fn new(machine: VotingMachine) -> Self {
        MemoryStore {
            voting_machine: machine,
            
        }
    }

   
    async fn get_voting_machine(&self) -> VotingMachine {
        self.voting_machine.clone() 
    }


    async fn put_voting_machine(&mut self, machine: VotingMachine) {
        self.voting_machine = machine;
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

	let mut voting_machine = VotingMachine::new(voters, scoreboard);
   
    

    let mut store = MemoryStore::new(voting_machine).await;
    

    let stored_machine = store.get_voting_machine().await;
    

    assert!(voting_machine==stored_machine);
}





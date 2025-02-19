use async_trait::async_trait;

use crate::{domain::voting_machine::VotingMachine, storage::Storage};

#[derive(Clone)]
pub struct MemoryStore {
	voting_machine: VotingMachine,
}

#[async_trait]
impl Storage for MemoryStore {
	async fn new(machine: VotingMachine) -> anyhow::Result<Self> {
		Ok(Self {
			voting_machine: machine,
		})
	}

	async fn get_voting_machine(&self) -> anyhow::Result<VotingMachine> {
		Ok(self.voting_machine.clone())
	}

	async fn put_voting_machine(&mut self, machine: VotingMachine) -> anyhow::Result<()> {
		self.voting_machine = machine;
		Ok(())
	}
}

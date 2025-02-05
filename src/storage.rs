use async_trait::async_trait;

use crate::domain::voting_machine::VotingMachine;

#[async_trait]
pub trait Storage
where
	Self: Sized,
{
	async fn new(machine: VotingMachine) -> anyhow::Result<Self>;
	async fn get_voting_machine(&self) -> anyhow::Result<VotingMachine>;
	async fn put_voting_machine(&mut self, machine: VotingMachine) -> anyhow::Result<()>;
}

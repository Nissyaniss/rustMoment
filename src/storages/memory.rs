use std::collections::BTreeMap;

use async_trait::async_trait;

use crate::{domain::voting_machine::VotingMachine, storage::Storage};

struct MemoryStore {
	voting_machine: VotingMachine,
}

#[async_trait]
impl Storage for MemoryStore {}

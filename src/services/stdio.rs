use async_trait::async_trait;
use tokio::io::{self, AsyncBufReadExt, BufReader};

use crate::{
	interfaces::{cli_interfaces::handle_line, lexicon::Lexicon},
	service::Service,
	storage::Storage,
	use_cases::VotingController,
};

pub struct StdioService<Store> {
	lexicon: Lexicon,
	controller: VotingController<Store>,
}

#[async_trait]
impl<Store: Storage + Send + Sync> Service<Store> for StdioService<Store> {
	fn new(_port: u16, lexicon: Lexicon, controller: VotingController<Store>) -> Self {
		Self {
			lexicon,
			controller,
		}
	}
	async fn serve(&mut self) -> Result<(), anyhow::Error> {
		loop {
			let mut lines = BufReader::new(io::stdin()).lines();
			if let Some(line) = lines.next_line().await? {
				println!(
					"{}",
					handle_line(&line, &mut self.controller, &self.lexicon).await?
				);
			}
		}
	}
}

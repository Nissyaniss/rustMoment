use async_trait::async_trait;
use tokio::{
	io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
	net::TcpListener,
};

use crate::{
	interfaces::{
		cli_interfaces::handle_line,
		lexicon::{self, Lexicon},
	},
	service::Service,
	storage::Storage,
	use_cases::VotingController,
};

pub struct TcpService<Store> {
	port: u16,
	lexicon: Lexicon,
	controller: VotingController<Store>,
}

#[async_trait]
impl<Store: Storage + Send + Sync + Clone + 'static> Service<Store> for TcpService<Store> {
	fn new(port: u16, lexicon: Lexicon, controller: VotingController<Store>) -> Self {
		Self {
			port,
			lexicon,
			controller,
		}
	}

	async fn serve(self) -> Result<(), anyhow::Error> {
		let server_endpoint = format!("127.0.0.1:{}", self.port);
		let listener = TcpListener::bind(server_endpoint).await?;
		loop {
			let (stream, _) = listener.accept().await?;
			let controller = self.controller.clone();
			let lexicon = self.lexicon.clone();
			tokio::spawn(async move {
				let (reader, mut writer) = stream.into_split();
				let mut lines = BufReader::new(reader).lines();
				while let Ok(Some(line)) = lines.next_line().await {
					writer
						.write_all(
							handle_line(&line, controller.clone(), &lexicon)
								.await?
								.as_bytes(),
						)
						.await?;
					writer.flush().await?;
				}
				return Ok::<(), anyhow::Error>(());
			});
		}
	}
}

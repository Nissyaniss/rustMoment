use async_trait::async_trait;
use tokio::net::UdpSocket;

use crate::{
	interfaces::{cli_interfaces::handle_line, lexicon::Lexicon},
	service::Service,
	storage::Storage,
	use_cases::VotingController,
};

pub struct UdpService<Store> {
	port: u16,
	lexicon: Lexicon,
	controller: VotingController<Store>,
}

#[async_trait]
impl<Store: Storage + Send + Sync + Clone> Service<Store> for UdpService<Store> {
	fn new(port: u16, lexicon: Lexicon, controller: VotingController<Store>) -> Self {
		Self {
			port,
			lexicon,
			controller,
		}
	}

	async fn serve(self) -> Result<(), anyhow::Error> {
		let server_endpoint = format!("127.0.0.1:{}", self.port);
		let socket = UdpSocket::bind(server_endpoint).await?;
		let mut buf = vec![0; 1000];
		loop {
			let (len, src) = socket.recv_from(&mut buf).await?;
			let message = String::from_utf8_lossy(&buf[..len - 1]);
			socket
				.send_to(
					handle_line(&message, self.controller.clone(), &self.lexicon)
						.await?
						.as_bytes(),
					&src,
				)
				.await?;
		}
	}
}

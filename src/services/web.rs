use std::net::SocketAddr;

use async_trait::async_trait;
use axum::Router;

use crate::{
	interfaces::lexicon::Lexicon, service::Service, storage::Storage, use_cases::VotingController,
};

pub struct WebService {
	address: SocketAddr,
	router: Router,
}

#[async_trait]
impl<Store: Storage + Send + Sync + Clone + 'static> Service<Store> for WebService {
	fn new(port: u16, lexicon: Lexicon, controller: VotingController<Store>) -> Self {
		todo!();
	}

	async fn serve(self) -> Result<(), anyhow::Error> {
		let listener = tokio::net::TcpListener::bind(&self.address).await.unwrap();
		axum::serve(listener, self.router.clone()).await.unwrap();
		Ok(())
	}
}

use clap::Parser;
use rustMoment::{app_builder::run_app, configuration::Configuration};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let configuration = Configuration::parse();
	run_app(configuration).await
}

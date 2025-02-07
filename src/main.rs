use clap::Parser;
use rustMoment::{
	app_builder::{handle_lines, run_app},
	configuration::{Configuration, StoredType},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let configuration = Configuration::parse();
	run_app(configuration).await
}

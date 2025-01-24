use clap::Parser;

#[derive(Parser)]
pub struct Configuration {
	#[arg(short, long, required = true, num_args = 1..)]
	pub candidates: Vec<String>,
}

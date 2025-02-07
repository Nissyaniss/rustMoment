use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
pub struct Configuration {
	#[arg(short, long, required = true, num_args = 1..)]
	pub candidates: Vec<String>,

	#[arg(short, long, required = true)]
	pub storage: StoredType,
}

#[derive(Clone, Copy, ValueEnum, Debug)]
pub enum StoredType {
	File,
	Memory,
}

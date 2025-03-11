use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
pub struct Configuration {
	#[arg(short, long, required = true, num_args = 1..)]
	pub candidates: Vec<String>,

	#[arg(short, long, required = true)]
	pub storage: StoredType,

	#[arg(short, long, required = true)]
	pub language: LanguageType,

	#[arg(long, required = true)]
	pub service: ServiceType,

	#[arg(short, long, required = true)]
	pub port: u16,
}

#[derive(Clone, Copy, ValueEnum, Debug)]
pub enum ServiceType {
	Stdio,
	Udp,
	Tcp,
	Web,
}

#[derive(Clone, Copy, ValueEnum, Debug)]
pub enum StoredType {
	File,
	Memory,
}

#[derive(Clone, Copy, ValueEnum, Debug)]
pub enum LanguageType {
	Fr,
	En,
}

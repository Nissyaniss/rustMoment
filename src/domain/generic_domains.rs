use std::{collections::BTreeSet, fmt::Display};

#[derive(Ord, PartialEq, PartialOrd, Eq, Clone, Debug)]
pub struct Voter(pub String);

impl Display for Voter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

#[derive(Ord, PartialEq, PartialOrd, Eq, Clone, Debug)]
pub struct Candidate(pub String);

impl Display for Candidate {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

#[derive(Ord, PartialEq, PartialOrd, Eq, Clone, Copy, Default, Debug)]
pub struct Score(pub usize);

impl Display for Score {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AttendenceSheet(pub BTreeSet<Voter>);

impl Default for AttendenceSheet {
	fn default() -> Self {
		let set: BTreeSet<Voter> = BTreeSet::new();
		Self(set)
	}
}

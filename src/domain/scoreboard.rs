use std::collections::BTreeMap;

use super::generic_domains::{Candidate, Score};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Scoreboard {
	pub scores: BTreeMap<Candidate, Score>,
	pub blank_score: Score,
	pub invalid_score: Score,
}

impl Scoreboard {
	#[must_use]
	pub fn new(candidates: Vec<Candidate>) -> Self {
		let mut scores: BTreeMap<Candidate, Score> = BTreeMap::default();
		for candidate in candidates {
			scores.insert(candidate, Score::default());
		}

		Self {
			scores,
			blank_score: Score(0),
			invalid_score: Score(0),
		}
	}
}

use std::collections::BTreeMap;

use super::generic_domains::{Candidate, Score};

pub struct Scoreboard {
	pub scores: BTreeMap<Candidate, Score>,
	pub blank_score: Score,
	pub invalid_score: Score,
}

impl Scoreboard {
	pub fn new(candidates: Vec<Candidate>) -> Self {
		let mut scores: BTreeMap<Candidate, Score> = BTreeMap::default();
		for candidate in candidates {
			scores.insert(candidate, Score(0));
		}

		Self {
			scores,
			blank_score: Score(0),
			invalid_score: Score(0),
		}
	}
}

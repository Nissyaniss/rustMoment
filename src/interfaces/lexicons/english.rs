use crate::interfaces::lexicon::Lexicon;

impl Lexicon {
	pub const fn english() -> Self {
		Self {
			blank_vote: "has voted blank.",
			has_already_voted: "has already voted.",
			accepted_vote: "has voted for",
			invalid_vote: "has voted null.",
			voters: "voters",
			vote: "vote",
			scores: "scores",
			invalid_command: "Invalid command\n",
			scores_title: "Scores:\n",
			voters_title: "Voters:\n",
			blank: "Blank",
			invalid: "Invalid",
			help: "Help :\n - vote <name> [candidate]\n - scores\n - voters",
			candidate_missing: "Voter missing.",
			vote_machine: "Voting Machine",
			urn: "Urn",
			voter: "Voter",
			candidate: "Candidate",
			Scores: "Scores",
			Voters: "Voters",
		}
	}
}

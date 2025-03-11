use crate::interfaces::lexicon::Lexicon;

impl Lexicon {
	pub const fn french() -> Self {
		Self {
			blank_vote: "a voter blanc.",
			has_already_voted: "a deja voter.",
			accepted_vote: "a voter pour",
			invalid_vote: "a voter null.",
			voters: "votants",
			vote: "voter",
			scores: "scores",
			blank: "Blanc",
			invalid: "Null",
			invalid_command: "Commande non valide\n",
			scores_title: "Voici les scores:\n",
			voters_title: "Voici les votants:\n",
			help: "Aide :\n - voter <nom> [candidat]\n - scores\n - votants",
			candidate_missing: "Il manque un votant.",
			vote_machine: "Machine de vote",
			urn: "Urne",
			voter: "Votant",
			candidate: "Candidat",
			Scores: "Scores",
			Voters: "Votants",
		}
	}
}

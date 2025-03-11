use maud::{html, Markup};

use crate::{domain::voting_machine::VotingMachine, interfaces::lexicon::Lexicon};

use super::web_routes::WebRoutes;

pub fn vote_form(routes: &WebRoutes, lexicon: &Lexicon) -> Markup {
	html!(
		script src="https://unpkg.com/htmx.org@1.9.2" {}
		h1 #title { (lexicon.vote_machine) }
		h2 #urne { (lexicon.urn) }
		div #inputs {
			div #votant {
				p { (lexicon.voter) }
				input #input_votant;
			}
			div #candidat {
				p { (lexicon.candidate) }
				input #input_candidat;
			}
		}
		button { (lexicon.vote) }
	)
}

pub fn voting_machine(routes: &WebRoutes, lexicon: &Lexicon, machine: &VotingMachine) -> Markup {
	html!(
		h2 #scores_title { (lexicon.Scores) }
		div #scores {
			@for (candidate, score) in &machine.get_scoreboard().scores {
				p { (candidate) (score) }
			}
			p { (&machine.get_scoreboard().blank_score) }
			p { (&machine.get_scoreboard().invalid_score) }
		}
		div #voters {
			ul {
				@for voter in &machine.get_voter().0 {
					li { (voter) }
				}
			}
		}
	)
}

pub fn index(routes: &WebRoutes, lexicon: &Lexicon, machine: &VotingMachine) -> Markup {
	let form = vote_form(routes, lexicon);
	let machine = voting_machine(routes, lexicon, machine);

	html!((form)(machine))
}

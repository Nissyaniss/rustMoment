use std::collections::BTreeMap;
use tokio::io::{self, AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let mut tableau_candidats = BTreeMap::new();

	tableau_candidats.insert("NixOS".to_string(), 0u32);
	tableau_candidats.insert("Windows".to_string(), 0u32);
	tableau_candidats.insert("MacOS".to_string(), 0u32);
	tableau_candidats.insert("Nul".to_string(), 0u32);
	tableau_candidats.insert("Blanc".to_string(), 0u32);

	let mut tableau_votants = BTreeMap::new();

	loop {
		let mut lines = BufReader::new(io::stdin()).lines();
		if let Some(line) = lines.next_line().await? {
			let mut mots = line.split(' ');
			let premier_mot = mots.next().unwrap_or_default();
			let deuxieme_mot = mots.next().unwrap_or_default();
			let troisieme_mot = mots.next().unwrap_or_default();
			match premier_mot {
				"voter" => {
					let res = voter(
						deuxieme_mot,
						troisieme_mot,
						&tableau_candidats,
						&tableau_votants,
					);
					if !res.0.is_empty() && !res.1.is_empty() {
						tableau_votants.insert(res.0, true);
						if let Some(candidat) = tableau_candidats.get_mut(&res.1) {
							*candidat += 1;
						}
					}
				}
				"votants" => afficher_votants(tableau_votants.clone()),
				"scores" => afficher_score(tableau_candidats.clone()),
				_ => println!("Command invalide"),
			}
		}
	}
}

fn voter(
	deuxieme_mot: &str,
	troisieme_mot: &str,
	candidats: &BTreeMap<String, u32>,
	votants: &BTreeMap<String, bool>,
) -> (String, String, u32) {
	if deuxieme_mot.is_empty() {
		println!("Erreur nom de votant manquant :\nvoter <nom_votant> <nom_candidat>.");
	} else if votants.contains_key(deuxieme_mot) {
		println!("{deuxieme_mot} a deja vote.");
	} else if troisieme_mot.is_empty() {
		println!("{deuxieme_mot} a voter blanc.");
		return (deuxieme_mot.to_string(), "Blanc".to_string(), 1);
	} else if candidats.contains_key(troisieme_mot) {
		println!("{deuxieme_mot} a voter {troisieme_mot}.");
		return (deuxieme_mot.to_string(), troisieme_mot.to_string(), 1);
	} else {
		println!("{deuxieme_mot} a voter nul.");
		return (deuxieme_mot.to_string(), "Nul".to_string(), 1);
	}
	(String::new(), String::new(), 0)
}

fn afficher_score(scores: BTreeMap<String, u32>) {
	println!("\nVoici les scores\n");
	for (nom, score) in scores {
		println!("{nom}: {score}");
	}
}

fn afficher_votants(votants: BTreeMap<String, bool>) {
	println!("\nVoici les votants\n");
	for (votant, _) in votants {
		println!("{votant}");
	}
}

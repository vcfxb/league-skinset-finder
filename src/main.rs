
use std::collections::{HashMap, HashSet};
use scraper::{Html, Selector};
use skinsets::Skinsets;

mod lanes;
mod skinsets;

/// Player struct used to check for champ overlaps. 
struct Player {
    name: &'static str,
    champs: &'static [&'static str],
}

const MADDIE: Player = Player {
    name: "Maddie",
    champs: &[
        "Caitlyn",
        "Jinx",
        "Ashe",
        "Jhin"
    ]
};

const TONI: Player = Player {
    name: "Toni",
    champs: &[
        "Vel'Koz",
        "Evelynn",
        "Cho'Gath",
        "Briar",
        "Morgana",
    ]
};

const VENUS: Player = Player {
    name: "Venus",
    champs: &[
        "Mordekaiser",
        "Blitzcrank",
        "Lux",
        "Pantheon",
        "Illaoi"
    ],
};

const EMMA: Player = Player {
    name: "Emma",
    champs: &[
        "Diana",
        "Pyke",
        "Akali",
        "Fizz",
        "Ahri",
        "Jinx",
        "Kalista",
        "LeBlanc",
        "Lux",
        "Gwen",
        "Ezreal",
        "Soraka",
        "Renata Glasc",
        "Yuumi",
        "Seraphine",
        "Kindred",
        "Irelia",
        "Azir",
        "Kai'Sa",
        "Karma",
        "Kennen",
        "Mordekaiser",
        "Nami",
        "Quinn",
        "Senna",
        "Sivir",
        "Shyvana",
        "Taliyah",
        "Varus",
        "Viego",
        "Xayah"
    ],
};

const SKINSET_BLACKLIST: &'static [&'static str] = &[
    // Blacklisted for being aesthetically incoherent
    "Legacy", 
    // Blacklisted for being ugly. 
    "Battlecast"
];

const PLAYERS: &'static [Player] = &[MADDIE, TONI, VENUS, EMMA];

/// Make an iterator over all combinations of champions.
fn all_champ_combinations(players: &[Player]) -> Vec<Vec<&'static str>> {
    match players.len() {
        0 => Vec::new(),
        1 => players[0].champs.iter().map(|champ| vec![*champ]).collect(),
        _ => {
            // Get a list of all champ combinataions not including the first player. 
            let others = all_champ_combinations(&players[1..]);
            // Make a list to copy resuls into. 
            let mut result = Vec::new();

            for champ in players[0].champs {
                for champ_combo in others.iter() {
                    if !champ_combo.contains(champ) {
                        let mut new_combo = champ_combo.clone();
                        new_combo.insert(0, *champ);
                        result.push(new_combo);
                    }
                }
            }

            result
        }
    }
}

fn main() -> anyhow::Result<()> {
    // Make a new map to keep track of skinsets. 
    let skinset_map: Skinsets = Skinsets::new();
    
    // Iterate over every possible combination of champs for the given players.
    for champ_combo in all_champ_combinations(PLAYERS) {
        // Get the set of overlapping skinsets for the champions.
        let overlapping_skinsets: HashSet<String> = skinset_map.get_overlapping_skinsets(&champ_combo);

        if !overlapping_skinsets.is_empty() {
            // Match the champ to the player and print. 
            let prefix = champ_combo
                .iter()
                .zip(PLAYERS)
                .map(|(champ, player)| format!("{} plays {}", player.name, *champ))
                .collect::<Vec<_>>()
                .join(", ");

            println!("{prefix} -- overlapping skinsets: {overlapping_skinsets:?}");
        }
    }

    Ok(())
}

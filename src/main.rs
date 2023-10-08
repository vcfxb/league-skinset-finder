
use std::collections::HashSet;
use lanes::Lane;
use skinsets::Skinsets;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::Table;

use crate::lanes::LanesMap;

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
fn all_champ_combinations(players: &[Player], lanes_map: &LanesMap) -> Vec<Vec<(&'static str, Lane)>> {
    match players.len() {
        // No players - no combos.
        0 => Vec::new(),

        // One player -- suggest any of their champs in any available lane. 
        1 => {
            // Create new result vec to populate. 
            let mut result: Vec<Vec<(&'static str, Lane)>> = Vec::new();

            // Iterate over all the champs for the one player.
            for champ in players[0].champs {
                // Iterate over the lanes for a champ
                for lane in lanes_map.lanes_for_champ(*champ) {
                    result.push(vec![(*champ, lane)]);
                }
            }

            result
        },

        _ => {
            // Get a list of all champ combinataions not including the first player. 
            let others: Vec<Vec<(&'static str, Lane)>> = all_champ_combinations(&players[1..], lanes_map);
            // Make a list to copy resuls into. 
            let mut result = Vec::new();

            // Iterate over all the champs a player could play.
            for champ in players[0].champs {
                // Iterate over all the lanes the champ could play. 
                for lane in lanes_map.lanes_for_champ(*champ) {
                    // Iterate over all the other champ combos for the rest of the team. 
                    for champ_combo in others.iter() {
                        // Check if this champ is already in the combo
                        let contains_champ: bool = champ_combo.iter().find(|(c, _)| c == champ).is_some();
                        // Check if this lane is already covered in the combo.
                        let lane_covered: bool =  champ_combo.iter().find(|(_, l)| *l == lane).is_some();

                        // If neither are true then we can make a new combo using this champ in this lane for this 
                        // player.
                        if !contains_champ && !lane_covered {
                            let mut new_combo = champ_combo.clone();
                            new_combo.insert(0, (*champ, lane));
                            result.push(new_combo);
                        }
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
    // Make a new map to keep track of lanes available to champs.
    let lanes_map: LanesMap = LanesMap::new();
    // Make a table for printing things to the terminal.
    let mut table: Table = Table::new();
    // Add nice character styling to the table.
    table.load_preset(UTF8_FULL).apply_modifier(UTF8_ROUND_CORNERS);
    // Set the table header -- list of player names and skinset column.
    table.set_header(PLAYERS.iter().map(|p| p.name).chain(std::iter::once("Skinsets")));

    // Iterate over every possible combination of champs for the given players.
    for champ_combo in all_champ_combinations(PLAYERS, &lanes_map) {
        // Get the set of overlapping skinsets for the champions.
        let overlapping_skinsets: HashSet<String> = skinset_map.get_overlapping_skinsets(&champ_combo);

        // If there are overlapping skins, add a row to the table.
        if !overlapping_skinsets.is_empty() {
            // Transform the champ combo into an iterator over printable strings.
            let main_cols = champ_combo
                .into_iter()
                .map(|(champ, lane)| format!("{champ} in {lane}"));
            
            // Collect all the skinsets into a string we can print.
            let last_col = overlapping_skinsets
                .iter()
                .map(|skinset| format!("`{skinset}`"))
                .collect::<Vec<_>>()
                .join(",");

            // Add a row to the table.
            table.add_row(main_cols.chain(std::iter::once(last_col)));
        }
    }

    // Print the table.
    println!("{table}");
    // Exit status OK. 
    Ok(())
}

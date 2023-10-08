
use std::collections::HashSet;
use skinsets::Skinsets;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::Table;

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
        // "Ashe",
        // "Jhin"
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
    // Make a table for printing things to the terminal.
    let mut table: Table = Table::new();
    // Add nice character styling to the table.
    table.load_preset(UTF8_FULL).apply_modifier(UTF8_ROUND_CORNERS);
    // Set the table header -- list of player names and skinset column.
    table.set_header(PLAYERS.iter().map(|p| p.name).chain(std::iter::once("Skinsets")));

    // Iterate over every possible combination of champs for the given players.
    for champ_combo in all_champ_combinations(PLAYERS) {
        // Get the set of overlapping skinsets for the champions.
        let overlapping_skinsets: HashSet<String> = skinset_map.get_overlapping_skinsets(&champ_combo);

        // If there are overlapping skins, add a row to the table.
        if !overlapping_skinsets.is_empty() {
            // Collect all the skinsets into a string we can print.
            let last_col = overlapping_skinsets
                .iter()
                .map(|skinset| format!("`{skinset}`"))
                .collect::<Vec<_>>()
                .join(",");

            // Add a row to the table.
            table.add_row(champ_combo.iter().chain(std::iter::once(&last_col.as_str())));
        }
    }

    // Print the table.
    println!("{table}");
    // Exit status OK. 
    Ok(())
}


use instant::Instant;
use lanes::Lane;
use log::{Level, info};
use players::Player;
use skinsets::Skinsets;
// use comfy_table::modifiers::UTF8_ROUND_CORNERS;
// use comfy_table::presets::UTF8_FULL;
// use comfy_table::Table;
use lanes::LanesMap;

use crate::components::App;
use crate::players::PLAYERS;

use crate::lanes::GLOBAL_LANES_MAP;
// use crate::skinsets::

// // Include the file generated by the build script. 
// include!(concat!(env!("OUT_DIR"), "/generated.rs"));

mod lanes;
mod skinsets;
mod players;
mod resolver;
mod components;


/// Make a list of all combinations of champions.
fn all_champ_combinations(players: &[Player]) -> Vec<Vec<(&'static str, Lane)>> {
    match players.len() {
        // Base case here should be len() == 1
        0 => panic!("Should have at least one champion,"),

        // One player -- suggest any of their champs in any available lane. 
        1 => {
            // Create new result vec to populate. 
            let mut result: Vec<Vec<(&'static str, Lane)>> = Vec::new();

            // Iterate over all the champs for the one player.
            for champ in players[0].champs {
                // Iterate over the lanes for a champ
                for lane in GLOBAL_LANES_MAP.with(|map| map.lanes_for_champ(*champ)) {
                    result.push(vec![(*champ, lane)]);
                }
            }

            result
        },

        _ => {
            // Get a list of all champ combinataions not including the first player. 
            let others: Vec<Vec<(&'static str, Lane)>> = all_champ_combinations(&players[1..]);
            // Make a list to copy resuls into. 
            let mut result = Vec::new();

            // Iterate over all the champs a player could play.
            for champ in players[0].champs {
                // Iterate over all the lanes the champ could play. 
                for lane in GLOBAL_LANES_MAP.with(|map| map.lanes_for_champ(*champ)) {
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

fn main() {
    // Track the starting instant for performance reasons. 
    let start: Instant = Instant::now();
    // Initialize the logger.
    console_log::init_with_level(Level::Info).expect("error initializing logger");
    info!("Logger started");
    // Force init of lazy thread local statics -- if the logger is set to `Debug` then this will almost certainly 
    // take forever -- the scraper generates tons of debug log messages. 
    GLOBAL_LANES_MAP.with(|_| {});
    GLOBAL_LANES_MAP.with(|_| {});
    // Save the time it took to intialize the maps and logger.
    let init_time: Instant = Instant::now();
    // Log how much time it took to scrape/initialize these globals.
    info!("Globals initialized in {:?}", init_time - start);
    // Start the yew global renderer -- this kicks off a loading bar to wait for everything to initialize. 
    yew::Renderer::<App>::new().render();

    /*
    // Make a table for printing things to the terminal.
    let mut table: Table = Table::new();
    // Add nice character styling to the table.
    table.load_preset(UTF8_FULL).apply_modifier(UTF8_ROUND_CORNERS);
    // Set the table header -- list of player names and skinset column.
    table.set_header(PLAYERS.iter().map(|p| p.name).chain(std::iter::once("Overlapping Skinsets")));

    // Iterate over every possible combination of champs for the given players.
    for champ_combo in all_champ_combinations(PLAYERS) {
        // Get the set of overlapping skinsets for the champions.
        let overlapping_skinsets: HashSet<String> = GLOBAL_SKINSETS_MAP.get_overlapping_skinsets(&champ_combo);

        // If there are overlapping skins, add a row to the table.
        if !overlapping_skinsets.is_empty() {
            // Transform the champ combo into an iterator over printable strings.
            let main_cols = champ_combo
                .into_iter()
                .map(|(champ, lane)| format!("{champ} {lane}"));
            
            // Collect all the skinsets into a string we can print.
            let last_col = overlapping_skinsets
                .iter()
                .map(|skinset| format!("{skinset}"))
                .collect::<Vec<_>>()
                .join(", ");

            // Add a row to the table.
            table.add_row(main_cols.chain(std::iter::once(last_col)));
        }
    }

    // Print the table.
    println!("{table}");
    println!("Resolved in {:?} (total time: {:?})", Instant::now() - init_time, Instant::now() - start);
    // Exit status OK. 
    Ok(())
    */
}

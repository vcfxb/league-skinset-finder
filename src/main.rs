
use instant::Instant;
use log::{Level, info};
use crate::components::App;
use crate::lanes::GLOBAL_LANES_MAP;
use crate::skinsets::GLOBAL_SKINSETS_MAP;

mod lanes;
mod skinsets;
mod players;
mod components;

fn main() {
    // Track the starting instant for performance reasons. 
    let start: Instant = Instant::now();
    // Initialize the logger.
    console_log::init_with_level(Level::Info).expect("error initializing logger");
    info!("Logger started");
    // Force init of lazy thread local statics -- if the logger is set to `Debug` then this will almost certainly 
    // take forever -- the scraper generates tons of debug log messages. 
    GLOBAL_LANES_MAP.with(|_| {});
    GLOBAL_SKINSETS_MAP.with(|_| {});
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

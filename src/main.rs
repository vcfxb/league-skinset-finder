
use std::collections::{HashMap, HashSet};
use scraper::{Html, Selector, ElementRef};

/// Include the wiki sets table from https://leagueoflegends.fandom.com/wiki/Champion_skin/Skin_themes.
#[allow(unused)]
const WIKI_SETS_TABLE: &'static str = include_str!("../assets/wiki-sets-table.html");

/// Include the official sets table from https://leagueoflegends.fandom.com/wiki/Champion_skin/Skin_themes. 
#[allow(unused)]
const OFFICIAL_SETS_TABLE: &'static str = include_str!("../assets/official-sets-table.html");

/// Select which table to use. 
const USE_TABLE: &'static str = WIKI_SETS_TABLE;

/// List of champions maddie plays.
const MADDIE_CHAMPS: &'static [&'static str] = &[
    "Caitlyn",
    "Jinx",
    "Ashe",
    "Jhin"
];

/// List of champions that toni plays.
const TONI_CHAMPS: &'static [&'static str] = &[
    "Vel'Koz",
    "Evelynn",
    "Cho'Gath",
    "Briar",
    "Samira"
];

fn main() -> anyhow::Result<()> {
    // Create maps to do bi-directional lookup of skins for champs. 
    let mut skin_sets_by_champ: HashMap<String, HashSet<String>> = HashMap::new();
    let mut champs_by_skinset: HashMap<String, HashSet<String>> = HashMap::new();

    // Parse the fragment we're useing.
    let document = Html::parse_fragment(USE_TABLE);

    // Parse a selector that will find us the main table. 
    let table_selector: Selector = Selector::parse("table.wikitable").expect("table selector good");

    // Get the table element
    let table: ElementRef = document
        .select(&table_selector)
        .next()
        .expect("finds wikitable");

    // Make a selector to get rows out of the table. 
    let rows_selector: Selector = Selector::parse("tr").expect("rows selcector good");

    // Make a selector to find champs from a row element. 
    let champs_selector: Selector = Selector::parse("li > span").expect("champ selector good");

    // Make a selector to find the set name from a row ref. 
    let set_name_selector: Selector = Selector::parse("th:last-of-type").expect("set name selector good");

    // Iterate over all the rows of the table.
    for row_ref in table.select(&rows_selector).skip(1) { // skip(1) to remove the header row
        // Get the set name. 
        let set_name: String = row_ref
            .select(&set_name_selector)
            .next()
            .expect("finds set name")
            .text()
            .collect::<String>();

        // Legacy is a pseudo-set for old skins.
        if set_name == "Legacy" { continue; }

        // Get an iterator over all the champ names in this set.
        let champs_iter = row_ref
            .select(&champs_selector)
            .map(|champ_el_ref| {
                champ_el_ref
                    // Get the referenced element
                    .value()
                    // Read the 'data-champion' attribute
                    .attr("data-champion")
                    // Crash if it's not there.
                    .expect("champion name available")
                    // Convert to owned string
                    .to_owned()
            });

        for champ in champs_iter {
            // Add the champ-skin relation 
            champs_by_skinset
                // Get the map entry for the skinset.
                .entry(set_name.clone())
                // If this skinset is unrecognized, add a default. 
                .or_default()
                // Add the champ to the skinset.
                .insert(champ.clone());

            skin_sets_by_champ
                // Get the map entry for this champ.
                .entry(champ)
                // Make a new empty one if not recognized.
                .or_default()
                // Add the set name to this champ's list. 
                .insert(set_name.clone());
        }   
    }

    // Find intersections
    for maddie in MADDIE_CHAMPS {
        // Get the skins for this champ.
        let maddie_skins = &skin_sets_by_champ[*maddie];

        for toni in TONI_CHAMPS {
            // Get skin sets for this champ.
            let toni_skins = &skin_sets_by_champ[*toni];

            // Get a list of overlapping skins.
            let intersection = maddie_skins
                .intersection(toni_skins)
                .collect::<Vec<&String>>();

            if !intersection.is_empty() {
                println!("Maddie plays {maddie}, Toni plays {toni}, overlapping skinsets: {intersection:?}");
            }
        }
    }

    Ok(())
}

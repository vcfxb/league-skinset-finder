//! Champ Skinset table calculations. 

use std::collections::{HashMap, HashSet};
use implicit_clone::unsync::IString;
use scraper::{Html, Selector};
use crate::lanes::Lane;

thread_local! {
    /// Global static skinsets map. 
    pub static GLOBAL_SKINSETS_MAP: Skinsets = Skinsets::new();
}

/// Include the wiki sets table from https://leagueoflegends.fandom.com/wiki/Champion_skin/Skin_themes.
#[allow(unused)]
const WIKI_SETS_TABLE: &'static str = include_str!("../assets/wiki-sets-table.html");

/// Include the official sets table from https://leagueoflegends.fandom.com/wiki/Champion_skin/Skin_themes. 
#[allow(unused)]
const OFFICIAL_SETS_TABLE: &'static str = include_str!("../assets/official-sets-table.html");

/// Select which table to use. 
const USE_TABLE: &'static str = WIKI_SETS_TABLE;

/// Map from champ name to set of skinset names available. 
#[derive(Debug)]
pub struct Skinsets {
    /// Map from champ name to the hash set of skinset names they can use.
    champ_to_skinset_map: HashMap<IString, HashSet<IString>>,

    /// Use a once-cell here to lazily resolve the set of all champs when needed. 
    set_of_all_skinsets: HashSet<IString>
}

impl Skinsets {
    /// Populate a new skinset map by scraping the downloaded HTML fragments. 
    fn new() -> Self {
        // Parse the fragment we're useing.
        let fragment = Html::parse_fragment(USE_TABLE);
        // Make a selector to get rows out of the table. 
        let rows_selector: Selector = Selector::parse("tr").expect("rows selector good");
        // Make a selector to find champs from a row element. 
        let champs_selector: Selector = Selector::parse("li > span").expect("champ selector good");
        // Make a selector to find the set name from a row ref. 
        let set_name_selector: Selector = Selector::parse("th:last-of-type").expect("set name selector good");
        // Make an iterator to go over all the rows of the skinset table, skipping the header row.
        let row_iter = fragment.select(&rows_selector).skip(1);
        // Make the champ-skinset map to populate
        let mut champ_to_skinset_map: HashMap<IString, HashSet<IString>> = HashMap::new();
        // Make set of all skinsets to store and pass out too. 
        let mut set_of_all_skinsets: HashSet<IString> = HashSet::new();

        // Iterate over all the rows of the table.
        for row_ref in row_iter { 
            // Get the set name. 
            let set_name: IString = row_ref
                .select(&set_name_selector)
                .next()
                .expect("finds set name")
                .text()
                .collect::<String>()
                .into();

            // Insert/upsert into the set of all skinsets.
            set_of_all_skinsets.insert(set_name.clone());

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
                        // Convert to IString
                        .into()
                });

            // Add all of the champ-skinset mappings into the map. 
            for champ in champs_iter {
                champ_to_skinset_map
                    // Get the map entry for this champ.
                    .entry(champ)
                    // Make a new empty one if not recognized.
                    .or_default()
                    // Add the set name to this champ's list. 
                    .insert(set_name.clone());
            }   
        }

        Self { champ_to_skinset_map, set_of_all_skinsets }
    }

    /// Get the set of skinsets shared by all champs in a list. Note that the set may be empty.  
    pub fn get_overlapping_skinsets(&self, champ_list: &[(IString, Lane)]) -> HashSet<IString> {
        // Start with all the skins the first champ can use.
        let mut intersection = self.champ_to_skinset_map[&champ_list[0].0].clone();

        // For each of the remaining champs, reduce the intersection to overlapping skinsets.
        for (champ, _) in champ_list.iter().skip(1) {
            // Get a reference to this champ's skinsets.
            let champ_skinsets = &self.champ_to_skinset_map[champ];
            // Clone and collect all the skinset names into the new intersection set. 
            intersection = intersection.intersection(champ_skinsets).cloned().collect();
        }

        intersection
    }

    /// Çlone a hash-set of all the skinsets in league currently. 
    pub fn all_skinsets(&self) -> HashSet<IString> {
        self.set_of_all_skinsets.clone()
    }
}

//! Champ lane table calculations.

use std::collections::HashMap;
use derive_more::Display;
use enumflags2::{BitFlags, bitflags};
use scraper::{Html, Selector};

/// Lane table from https://leagueoflegends.fandom.com/wiki/List_of_champions_by_draft_position. 
const LANES_HTML: &'static str = include_str!("../assets/champ-lanes-table.html");

/// Used to manually add lanes to champions that are not otherwise considered kosher. 
const MANUAL_LANE_OVERRIDES: &'static [(&'static str, Lane)] = &[
    ("Caitlyn", Lane::Top),
    ("Cho'Gath", Lane::Support),
];

/// Bitflaggable Lane enumeration for the available lanes in league. 
#[bitflags]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Display)]
pub enum Lane {
    Top,
    Jungle,
    Mid,
    Bot,
    Support
}

/// Map of champs to their playable lanes. 
#[derive(Debug)]
pub struct LanesMap {
    /// Map from champion to bitflags of playable lanes. 
    champ_to_lanes_map: HashMap<String, BitFlags<Lane>>
}
impl LanesMap {
    /// Construct a new [`LanesMap`] by scraping the downloaded HTML fragment and adding overrides. 
    pub fn new() -> Self {
        // Make map to populate.
        let mut champ_to_lanes_map = HashMap::new();
        // Load the table fragment into a scrapable document.
        let fragment: Html = Html::parse_fragment(LANES_HTML);
        // Make a selector to get rows out of the table. 
        let rows_selector: Selector = Selector::parse("tbody > tr").expect("rows selector good");
        // Make a selector to get columns out of a row element. 
        let cols_selector: Selector = Selector::parse("td").expect("cols selector good");

        // Iterate over every row in the table.
        for row_ref in fragment.select(&rows_selector) {
            // Assume the children of the row are (in order):
            // Champ name, top, jungle, mid, bot lane, support, unused. 

            // Make an iterator over column elements that takes the exact number we want. 
            let mut cols_iterator = row_ref
                .select(&cols_selector)
                // Ignore the unused column.
                .take(6);

            // Take the champ name from the iterator. 
            let champ_name: String = cols_iterator
                .next()
                .expect("Finds champ name element")
                .value()
                .attr("data-sort-value")
                .expect("Finds champ name attribute")
                .to_owned();

            // Make a lane bitflags to populate by iterating over the rest of the columns. 
            let mut lanes: BitFlags<Lane> = BitFlags::<Lane>::empty();
            // Iterate over the remaining columns to populate bitflags.
            for (index, col) in cols_iterator.enumerate() {
                if col.value().attr("data-sort-value").is_some() {
                    match index {
                        0 => lanes |= Lane::Top,
                        1 => lanes |= Lane::Jungle,
                        2 => lanes |= Lane::Mid,
                        3 => lanes |= Lane::Bot,
                        4 => lanes |= Lane::Support,
                        // Unreachable because we limit the number of <td> tags in the iterator using take()
                        _ => unreachable!()
                    }
                }
            }

            // Check if there are any lane overrides.
            for (champ, lane) in MANUAL_LANE_OVERRIDES {
                if *champ == champ_name {
                    lanes |= *lane;
                }
            }

            // Add the champ and their lanes to the map. We use insert rather than upsert here because we assume there
            // are no duplicates in the table. 
            champ_to_lanes_map.insert(champ_name, lanes);
        }

        Self { champ_to_lanes_map }
    }

    /// Get an iterator over all the lanes available for a given champion. 
    pub fn lanes_for_champ(&self, champ_name: &str) -> impl Iterator<Item = Lane> {
        self.champ_to_lanes_map[champ_name].iter()
    }
}

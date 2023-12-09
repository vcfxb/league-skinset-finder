//! Champ Skinset table calculations. 

use std::collections::{HashMap, HashSet};
use implicit_clone::unsync::IString;
use crate::lanes::Lane;
use crate::generated::{ALL_SKINSET_NAMES, CHAMPS_TO_SKINSETS};

/// Skinsets we're not playing for various reasons. 
const SKINSET_BLACKLIST: &'static [&'static str] = &[
    // Blacklisted for being aesthetically incoherent
    "Legacy", 
    // Blacklisted for being ugly. 
    "Battlecast",
    // Many ugly skins
    "Infernal"
];

thread_local! {
    /// Global static skinsets map. 
    pub static GLOBAL_SKINSETS_MAP: Skinsets = Skinsets::new();
}


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
        // Make results objects.
        let mut champ_to_skinset_map = HashMap::with_capacity(170);
        let mut set_of_all_skinsets = HashSet::new();

        // Iterate on the generated constant. 
        for skinset in ALL_SKINSET_NAMES {
            set_of_all_skinsets.insert(IString::Static(*skinset));
        }

        for (champ_name, skinsets_list) in CHAMPS_TO_SKINSETS {
            // Collect skinsets to hash set
            let skinsets_set = skinsets_list
                .iter()
                .map(|skinset| IString::Static(*skinset))
                .collect::<HashSet<_>>();

            champ_to_skinset_map.insert(IString::Static(*champ_name), skinsets_set);
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

//! Champ lane table calculations.

use std::collections::HashMap;
use enumflags2::BitFlags;
use implicit_clone::unsync::IString;
use crate::generated::LANE_DATA;

/// Re-export the generated lanes enum. 
pub use crate::generated::Lane;

thread_local! {
    /// Global map relating champions to their playable lanes. 
    pub static GLOBAL_LANES_MAP: LanesMap = LanesMap::new();
}



/// Map of champs to their playable lanes. 
#[derive(Debug)]
pub struct LanesMap {
    /// Map from champion to bitflags of default playable lanes. 
    champ_to_lanes_map: HashMap<IString, BitFlags<Lane>>,
}

impl LanesMap {
    /// Construct a new [`LanesMap`] by scraping the downloaded HTML fragment and adding overrides. 
    fn new() -> Self {
        // Make results map
        let mut champ_to_lanes_map = HashMap::with_capacity(170);

        // Fill results map. 
        for (champ_name, lanes) in LANE_DATA {
            champ_to_lanes_map.insert(IString::Static(*champ_name), *lanes);
        }

        Self { champ_to_lanes_map }
    }

    /// Get the lanes a champ is playable in. 
    pub fn lanes_for_champ(&self, champ_name: &str) -> BitFlags<Lane> {
        self.champ_to_lanes_map[champ_name]
    }

    /// Get a list of the name of every champ in the game. 
    pub fn all_champs(&self) -> Vec<&IString> {
        self.champ_to_lanes_map.keys().collect()
    }
}

//! Frontend models. 

use serde::{Serialize, Deserialize};
use crate::{constants::{ChampId, Lane}, generated::LANE_DATA};
use enumflags2::BitFlags;

/// State persisted for each player in the frontend.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PlayerRecord {
    /// Player name (optional -- resolve with player number otherwise).
    pub name: Option<String>,

    /// List of champs and what lanes for them. This is in the order that they're in in the UI. 
    pub champs: Vec<(ChampId, BitFlags<Lane>)>,
}

impl PlayerRecord {
    /// Create a new player with no names, and an empty champ list. 
    pub fn new() -> Self {
        Self {
            name: None,
            champs: Vec::with_capacity(LANE_DATA.len()),
        }
    }

    /// Update the lanes for a champ for this player if that champ is in this players list of champs.
    /// Return `true` if the champ was updated and `false` if that champ is not selected on this player record.
    fn update_champ(&mut self, champ_id: ChampId, lanes: BitFlags<Lane>) -> bool {
        self.champs
            .iter_mut()
            .find(|(iter_champ_id, _)| *iter_champ_id == champ_id)
            .map(|(_, lanes_mut)| *lanes_mut = lanes)
            .is_some()
    }

    /// Remove a champ on this player. If that champ is not in the list of [`PlayerRecord::champs`], do nothing. 
    fn remove_champ(&mut self, champ_id: ChampId) {
        // Remove the champ if they already were in the list. 
        let remove_index = self.champs
            .iter()
            .enumerate()
            .find(|(_, (iter_champ_id, _))| *iter_champ_id == champ_id)
            .map(|(index, _)| index);
        
        if let Some(index) = remove_index {
            // Just use remove here to maintain order.
            self.champs.remove(index);
        }
    }
}

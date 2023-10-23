//! The results table component, used to render the skinsets resolved for the selected champs. 

use std::{rc::Rc, cell::RefCell, collections::HashSet};

use instant::Instant;
use yew::prelude::*;
use crate::{lanes::Lane, skinsets::{GLOBAL_SKINSETS_MAP, Skinsets}};
use super::PlayerRecord;


/// Get a list of every combination of champs that this set of players could queue. 
/// This list will match the order of the list of players stored in the app. 
/// 
/// Requires that the slice has length >= 1 or panics.
fn resolve_all_champ_combinations(players: &[PlayerRecord]) -> Vec<Vec<(AttrValue, Lane)>> {
    match players.len() {
        0 => unreachable!("This function requires at least one champ in the slice to call"),

        // One player -- suggest any of their champs in any available lane. 
        1 => {
            // Borrow the players champ list.
            let champs_list = players[0].champs.borrow();
            // Create new result vec to populate -- starting capacity is at least the number of 
            // champs for the player. 
            let mut result: Vec<Vec<(AttrValue, Lane)>> = Vec::with_capacity(champs_list.len());

            // Iterate over all the champs for the one player.
            for (champ, lanes) in champs_list.iter() {
                // Iterate over the lanes for a champ
                for lane in lanes.iter() {
                    result.push(vec![(champ.clone(), lane)]);
                }
            }

            result
        },

        _ => {
            // Get a list of all champ combinataions not including the first player. 
            let others: Vec<Vec<(AttrValue, Lane)>> = resolve_all_champ_combinations(&players[1..]);
            // Borrow the first player's champ list.
            let champ_list = players[0].champs.borrow();
            // Make a list to copy results into. 
            let mut result = Vec::new();

            // Iterate over all the champs a player could play.
            for (champ, lanes) in champ_list.iter() {
                // Iterate over all the lanes the champ could play. 
                for lane in lanes.iter() {
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
                            new_combo.insert(0, (champ.clone(), lane));
                            result.push(new_combo);
                        }
                    }
                }
            }

            result
        }
    }
}

/// Properties passed to the table.
#[derive(PartialEq, Properties)]
pub struct ResultsTableProps {
    /// The list of players with their champ selections. 
    pub players: Vec<PlayerRecord>,
    /// The set of skinsets to exclude from results. 
    pub skinsets_excluded: Rc<RefCell<HashSet<AttrValue>>>,
}

#[function_component(ResultsTable)]
pub fn results_table(props: &ResultsTableProps) -> Html {
    // Track the start instant so we can log resolution/render times. 
    let start = Instant::now();
    // Get an iterator over the champ-combinations that could be played. 
    let all_comps = resolve_all_champ_combinations(&props.players);
    // Log info on resolution speed. 
    log::info!("Resolved all champion combos in {:?}", Instant::now() - start);
    // Get an iterator over the champ combinations that filters out any with no-overlapping, non-excluded skinsets.
    let displayed_comps = all_comps
        .into_iter()
        // Add the set of overlapping 
        .map(|champ_combo: Vec<(AttrValue, Lane)>| { 
            (champ_combo, GLOBAL_SKINSETS_MAP.with(|s: Skinsets| s.get_overlapping_skinsets(&champ_combo)))
        })
        .filter(|(_, overlapping_skinsets)| {

        })

    html! {

    }
}

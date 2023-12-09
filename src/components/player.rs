//! Player components in the league skinset finder.

use super::button::Button;
use crate::lanes::{Lane, LanesMap, GLOBAL_LANES_MAP};
use champ::ChampSelection;
use enumflags2::BitFlags;
use name_field::Name;
use std::rc::Rc;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

mod champ;
mod champ_dropdown;
mod lanes_select;
mod name_field;

#[derive(Properties, PartialEq, Debug)]
pub struct PlayerProps {
    /// Player ID (index) used for connecting inputs and labels.
    pub id: usize,
    /// The default name of this player.
    pub name: Option<AttrValue>,
    /// List of champs this player plays.
    /// This should only be updated by the parent component.
    pub champs: Rc<Vec<(AttrValue, BitFlags<Lane>)>>,
    /// Callback to handle player name chane.
    pub on_name_change: Callback<String>,
    /// Whether the remove player button is enabled (there need to be more than 6 champs).
    pub enable_remove: bool,
    /// Callback called to rempove the player.
    pub on_remove: Callback<()>,
    /// Callback to be emitted when this component update's its player's champ list.
    pub on_champ_list_update: Callback<Vec<(AttrValue, BitFlags<Lane>)>>,
}

#[function_component(Player)]
pub fn player(props: &PlayerProps) -> Html {
    // Get an Rc'd list of all the other available champs.
    let other_available_champs = GLOBAL_LANES_MAP.with(|lanes_map: &LanesMap| {
        // Get a list of all champions
        let all_champs = lanes_map.all_champs();
        // Get a reference to the list of selected champs.
        let selected_champs = props.champs.as_slice();
        // Filter any selected champs out of the list of all champs.
        let mut filtered_champs = all_champs
            .into_iter()
            // Apply the filter
            .filter(|champ| {
                selected_champs
                    .iter()
                    .all(|(selected_champ, _)| selected_champ != *champ)
            })
            // Clone all the IStrings (cheap because they're reference counted).
            .cloned()
            // Collect into Vec
            .collect::<Vec<_>>();
        // Sort the list of filter champs alphabetically.
        filtered_champs.sort();
        // Return an Rc'd list
        Rc::new(filtered_champs)
    });

    // Make a callback to get triggered when changing champs.
    let change_champ_callback = {
        // Clone the Rc ref to the champs list.
        let champs = props.champs.clone();
        // Clone the callback to emit when a champ list updates.
        let callback = props.on_champ_list_update.clone();

        Callback::from(move |(old_champ, new_champ): (Option<AttrValue>, String)| {
            // Take the champ list to modify.
            let mut list = Rc::try_unwrap(champs.clone()).unwrap_or_else(|e| (*e).clone());
            // If there is a champ to replace, find and replace them.
            if let Some(entry) = list
                .iter_mut()
                .find(|(champ, _)| Some(champ) == old_champ.as_ref())
            {
                entry.0 = new_champ.into();
            } else {
                // Get the default lanes for the champ.
                let default_lanes =
                    GLOBAL_LANES_MAP.with(|lanes_map| lanes_map.lanes_for_champ(&new_champ));
                list.push((new_champ.into(), default_lanes));
            }

            // Emit a callback to update the parent.
            callback.emit(list)
        })
    };

    // Make a callback to get triggered when a champ's lanes change.
    let lane_change_callback = {
        // Clone the Rc ref to the champs list.
        let champs = props.champs.clone();
        // Clone the callback to emit when a champ list updates.
        let callback = props.on_champ_list_update.clone();

        Callback::from(
            move |(champ_name, new_lanes): (AttrValue, BitFlags<Lane>)| {
                // Get the current champ list.
                let mut list = Rc::try_unwrap(champs.clone()).unwrap_or_else(|e| (*e).clone());
                // Find the champ in the champ list.
                let entry = list
                    .iter_mut()
                    .find(|(champ, _)| champ == &champ_name)
                    .expect("found champ entry");

                // Update their lanes.
                entry.1 = new_lanes;
                // Emit the callback to re-render.
                callback.emit(list);
            },
        )
    };

    // Callback to be triggered when a champ is removed.
    let remove_champ_callback = {
        // Clone the Rc ref to the champs list.
        let champs = props.champs.clone();
        // Clone the callback to emit when a champ list updates.
        let callback = props.on_champ_list_update.clone();

        Callback::from(move |champ_name: AttrValue| {
            // Get the current champ list.
            let mut list = Rc::try_unwrap(champs.clone()).unwrap_or_else(|e| (*e).clone());

            // Get the index of the champion to remove from the list.
            let index_to_remove = list
                .iter()
                .enumerate()
                .find(|(_, (champ, _))| champ == &champ_name)
                .expect("Find champ to remove")
                .0;

            // Remove the champ at that index.
            list.remove(index_to_remove);
            // Emit the callback to update the parent.
            callback.emit(list);
        })
    };

    html! {
        <div class={"card mt-2 bg-light text-dark"}>
            <div class={"card-body row g-2 align-items-center w-100"}>
                <div class={"col-10"}>
                    // Name field and handling
                    <Name player_id={props.id} player_name={props.name.clone().unwrap_or_default()} onchange={props.on_name_change.clone()} />
                </div>
                <div class={"col-2"}>
                    // Remove player button.
                    <Button enable={props.enable_remove} on_click={props.on_remove.clone()} class={"btn btn-danger w-100 fs-5 py-2"} >
                        <Icon icon_id={IconId::BootstrapTrash} /> {" Remove Player"}
                    </Button>
                </div>
            </div>
            // Champ selectors.
            <ul class={"list-group list-group-flush"}>
                {
                    props.champs.as_slice().iter().map(|(champ_name, lanes)| html!{
                        <li class={"list-group-item"}>
                            <ChampSelection
                                change_champ_callback={change_champ_callback.clone()}
                                other_available_champs={other_available_champs.clone()}
                                lane_change_callback={lane_change_callback.clone()}
                                remove_champ_callback={remove_champ_callback.clone()}

                                selected_champ={
                                    Some((champ_name.clone(), *lanes))
                                }
                            />
                        </li>
                    }).collect::<Html>()
                }
                <li class={"list-group-item"}>
                    <ChampSelection
                        selected_champ={None}
                        change_champ_callback={change_champ_callback.clone()}
                        other_available_champs={other_available_champs.clone()}
                        // Leave the lane-change callback and remove champ callback no-ops
                        // because there should not be any lanes or champ data
                        // on an empty champ selector.
                        lane_change_callback={Callback::noop()}
                        remove_champ_callback={Callback::noop()}
                    />
                </li>
            </ul>
        </div>
    }
}

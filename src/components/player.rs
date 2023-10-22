//! Player components in the league skinset finder. 



use std::{rc::Rc, cell::RefCell};
use yew::prelude::*;
use enumflags2::BitFlags;
use crate::lanes::{Lane, GLOBAL_LANES_MAP, LanesMap};
use name_field::Name;
use super::button::Button;
use champ::ChampSelection;
use yew_icons::{Icon, IconId};

mod name_field;
mod champ;
mod lanes_select;
mod champ_dropdown;

#[derive(Properties, PartialEq, Debug)]
pub struct PlayerProps {
    /// Player ID (index) used for connecting inputs and labels. 
    pub id: usize,
    /// The default name of this player. 
    pub name: Option<AttrValue>,
    /// List of champs this player plays. 
    /// This should only be updated by the parent component. 
    pub champs: Rc<RefCell<Vec<(AttrValue, BitFlags<Lane>)>>>,
    /// Callback to handle player name chane. 
    pub on_name_change: Callback<String>,
    /// Whether the remove player button is enabled (there need to be more than 6 champs).
    pub enable_remove: bool,
    /// Callback called to rempove the player. 
    pub on_remove: Callback<()>,
    /// Callback to be emitted when this component update's its player's champ list. 
    pub on_champ_list_update: Callback<()>,
}


#[function_component(Player)]
pub fn player(props: &PlayerProps) -> Html {
    // Get an Rc'd list of all the other available champs.
    let other_available_champs = GLOBAL_LANES_MAP.with(|lanes_map: &LanesMap| {
        // Get a list of all champions
        let all_champs = lanes_map.all_champs();
        // Get a reference to the map of selected champs.
        let selected_champs_borrow = props.champs.borrow();
        // Filter any selected champs out of the list of all champs.
        let mut filtered_champs = all_champs.into_iter()
            // Apply the filter
            .filter(|champ| selected_champs_borrow.iter().all(|(selected_champ, _)| selected_champ != *champ))
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
            // Mutably borrow the champ list.
            let mut list_borrow = champs.borrow_mut();
            // If there is a champ to replace, find and replace them.
            if let Some(entry) = list_borrow.iter_mut().find(|(champ, _)| Some(champ) == old_champ.as_ref()) {
                entry.0 = new_champ.into();
            } else {
                // Get the default lanes for the champ. 
                let default_lanes = GLOBAL_LANES_MAP.with(|lanes_map| lanes_map.lanes_for_champ(&new_champ));
                list_borrow.push((new_champ.into(), default_lanes));
            }
            
            // Emit a callback to re-render.
            callback.emit(())
        })
    };

    // Make a callback to get triggered when a champ's lanes change. 
    let lane_change_callback = {
        // Clone the Rc ref to the champs list. 
        let champs = props.champs.clone();
        // Clone the callback to emit when a champ list updates.
        let callback = props.on_champ_list_update.clone();

        Callback::from(move |(champ_name, new_lanes): (AttrValue, BitFlags<Lane>)| {
            // Mutably borrow the champ list.
            let mut list_borrow = champs.borrow_mut();
            // Find the champ in the champ list.
            let entry = list_borrow
                .iter_mut()
                .find(|(champ, _)| champ == &champ_name)
                .expect("found champ entry");

            // Update their lanes.
            entry.1 = new_lanes;
            // Emit the callback to re-render.
            callback.emit(());  
        })
    };

    // Callback to be triggered when a champ is removed. 
    let remove_champ_callback = {
        // Clone the Rc ref to the champs list. 
        let champs = props.champs.clone();
        // Clone the callback to emit when a champ list updates.
        let callback = props.on_champ_list_update.clone();

        Callback::from(move |champ_name: AttrValue| {
            // Mutably borrow the champ list.
            let mut list_borrow = champs.borrow_mut();

            // Get the index of the champion to remove from the list.
            let index_to_remove = list_borrow
                .iter()
                .enumerate()
                .find(|(_, (champ, _))| champ == &champ_name)
                .expect("Find champ to remove")
                .0;

            // Remove the champ at that index. 
            list_borrow.remove(index_to_remove);
            // Emit the callback to re-render
            callback.emit(());
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
                    props.champs.borrow().iter().map(|(champ_name, lanes)| html!{
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


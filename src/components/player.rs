//! Player components in the league skinset finder. 



use std::{rc::Rc, collections::HashMap, cell::RefCell};
use yew::prelude::*;
use enumflags2::BitFlags;
use crate::lanes::Lane;
use name_field::Name;
use remove_player::RemoveButton;

mod name_field;
mod remove_player;
mod champ;

#[derive(Properties, PartialEq, Debug)]
pub struct PlayerProps {
    /// Player ID (index) used for connecting inputs and labels. 
    pub id: usize,
    /// The default name of this player. 
    pub name: Option<AttrValue>,
    /// Map of champs this player plays. 
    pub champs: Rc<RefCell<HashMap<AttrValue, BitFlags<Lane>>>>,
    /// Callback to handle player name chane. 
    pub on_name_change: Callback<String>,
    /// Whether the remove player button is enabled (there need to be more than 6 champs).
    pub enable_remove: bool,
    /// Callback called to rempove the player. 
    pub on_remove: Callback<()>
}


#[function_component(Player)]
pub fn player(props: &PlayerProps) -> Html {
    // Use state to track if this player is enabled or not. 
    let player_enabled = use_state(|| true);

    html! {
        <div class={"card p-4 mt-2 bg-light text-dark"}>
            <div class={"row g-2 align-items-center w-100"}>
                <div class={"col-10"}>
                    // Name field and handling 
                    <Name player_id={props.id} player_name={props.name.clone().unwrap_or_default()} onchange={props.on_name_change.clone()} />
                </div>
                <div class={"col-2"}>
                    // Remove player button.
                    <RemoveButton enable={props.enable_remove} on_click={props.on_remove.clone()} />
                </div> 
            </div>
            // Champ selectors.
            <div>
            </div>
        </div>
    }
}


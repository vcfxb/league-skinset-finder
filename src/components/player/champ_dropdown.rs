//! Champ-select card's dropdown component.

use std::rc::Rc;
use yew::{function_component, html, use_context, Callback, Html, Properties};
use crate::{components::app::{Players, PlayersAction}, constants::ChampId};
use super::PlayerIndex;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub champ_index: usize,
    pub available_champs: Rc<[ChampId]>
}

#[function_component]
pub fn ChampDropdown(props: &Props) -> Html {
    let player_index = use_context::<PlayerIndex>().expect("player provides ctx");
    let players = use_context::<Players>().expect("player provides ctx");
    let players_dispatch = use_context::<Callback<PlayersAction>>().expect("player provides ctx");

    let select_id = format!("player-{}-champ-dropdown-{}", player_index, props.champ_index);

    html! {
        <div class="form-floating">
            // <select ref={self.select_ref.clone()} id={select_id.clone()} class={"form-select"} aria-label={"Champion Selection"} {onchange} autocomplete="off">
            //     // If there is no selected champs
            //     if props.selected_champ.is_none() {
            //         <option selected={true} disabled={true}>
            //             {"Select a champion..."}
            //         </option>
            //     }

            //     // All champs in list (handle for selections)
            //     {
            //         all_listed_champs_alphabetical
            //             .iter()
            //             .map(|champ_name| {
            //                 html! {
            //                     <option selected={Some(champ_name) == props.selected_champ.clone().as_ref().as_ref()}>
            //                         {champ_name}
            //                     </option>
            //                 }
            //             })
            //             .collect::<Html>()
            //     }

            // </select>

            // <label for={select_id}> {"Select a champion..."} </label>
        </div>
    }
}


//! Component to select the set of champs a player enjoys.

use std::rc::Rc;

use super::PlayerIndex;
use crate::{
    components::app::{Players, PlayersAction},
    constants::ChampId,
};
use yew::{function_component, html, use_context, Callback, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props {
    /// Sentinel value of player.champs.len() as placeholder for "Add Champ" card.
    pub champ_index: usize,
}

#[function_component]
pub fn ChampSelectCard(props: &Props) -> Html {
    let player_index = use_context::<PlayerIndex>().expect("player provides ctx");
    let players = use_context::<Players>().expect("player provides ctx");
    let players_dispatch = use_context::<Callback<PlayersAction>>().expect("player provides ctx");

    // Get a list of available champs for this player, in alphabetical order, excluding the champs they currently
    // have selected.
    // By iterating over champ ids in order we are already naturally alphabetical.
    // Shove it into an Rc slice for easy cloning.
    let available_champs: Rc<[ChampId]> = ChampId::iter_all()
        // Remove any that are currently selected.
        .filter(|champ_id| {
            players.0.borrow()[player_index.0]
                .champs
                .iter()
                .find(|(selected_champ, _)| selected_champ == champ_id)
                .is_none()
        })
        .collect::<Vec<_>>()
        .into_boxed_slice()
        .into();

    html! {
        <div class={"card-body"}>
            // <ChampDropdown
            //     selected_champ={selected_champ_name.clone()}
            //     other_available_champs={other_available_champs.clone()}
            //     on_change={ ctx.link().callback(|new_champ| Msg::ChangeChamp { new_champ }) }
            // />

            // if selected_champ.is_some() {
            //     <div class={"my-1 align-items-center"}>
            //         <LaneSelect
            //             lanes={selected_champ.clone().unwrap().1}
            //             update_lanes_callback={ ctx.link().callback(|new_lanes| Msg::ChangeLanes { new_lanes }) }
            //         />
            //     </div>

            //     <Button class={"btn btn-danger w-100"} enable={true} on_click={ ctx.link().callback(|_| Msg::RemoveChamp) }>
            //         <Icon icon_id={IconId::BootstrapTrash} /> {" Remove "} {selected_champ_name.clone()}
            //     </Button>
            // }
        </div>
    }
}

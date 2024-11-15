//! Player components in the league skinset finder.

use crate::constants::{ChampId, Lane};

use super::{button::Button, App, app::{PlayerRecord, AppMsg}};
// use champ::ChampSelection;
use enumflags2::BitFlags;
use name_field::Name;
use std::rc::Rc;
use yew::{prelude::*, html::Scope};
use yew_icons::{Icon, IconId};

// mod champ;
mod champ_dropdown;
mod lanes_select;
mod name_field;


/// Player component. 
pub struct Player;

impl Player {
    /// Get the scope of the parent app component. 
    fn get_parent_app_scope(ctx: &Context<Self>) -> Scope<App> {
        ctx.link()
            .get_parent()
            .expect("found parent component")
            .downcast()
    }
}

pub enum PlayerMsg {
    /// Change this players name. 
    ChangeName {
        new_name: String,
    },

    /// Remove this player. 
    RemoveThisPlayer,

}

/// The properties passed to the player component. 
#[derive(Properties, PartialEq, Debug)]
pub struct PlayerProps {
    /// Player ID (index) used for connecting inputs and labels.
    pub id: usize,
}

impl Component for Player {
    type Message = PlayerMsg;

    type Properties = PlayerProps;

    fn create(ctx: &Context<Self>) -> Self {
        Player
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // Player Name change 
            PlayerMsg::ChangeName { new_name } => {
                Player::get_parent_app_scope(ctx)
                    .send_message(AppMsg::PlayerNameUpdate { index: ctx.props().id, new_name });

                // Do not re-render with name that user just input.
                false
            }

            // Player removal
            PlayerMsg::RemoveThisPlayer => {
                Player::get_parent_app_scope(ctx)
                    .send_message(AppMsg::RemovePlayer { player_index: ctx.props().id });

                // Do not re-render this player if it has been removed. 
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Get the parent app scope so we can list the available champs for this player. 
        let parent_app_scope = Player::get_parent_app_scope(ctx);
        // Get the component so that we can get the list of champs for this player. 
        let parent_app = parent_app_scope.get_component().expect("got parent app");
        // Get the player record for this player. 
        let player_record: &PlayerRecord = &parent_app.players[ctx.props().id];
        // Get the list (in order) of champions anmd their lanes selected by this player. 
        let champ_selections: &[(ChampId, BitFlags<Lane>)] = player_record.champs.as_slice();
        // Determine if this player is removable
        let enable_remove: bool = parent_app.players.len() > 1;
        // Resolve the name of this player.
        let player_name: AttrValue = player_record.name.clone().unwrap_or_default();

        html! {
            <div class={"card mt-2 bg-light text-dark"}>
                <div class={"card-body row g-2 align-items-center w-100"}>
                    <div class={"col-10"}>
                        // Name field and handling
                        <Name player_id={ctx.props().id} player_name={player_name} onchange={ctx.link().callback(|new_name| PlayerMsg::ChangeName { new_name })} />
                    </div>
                    <div class={"col-2"}>
                        // Remove player button.
                        <Button enable={enable_remove} on_click={ctx.link().callback(|_| PlayerMsg::RemoveThisPlayer)} class={"btn btn-danger w-100 fs-5 py-2"} >
                            <Icon icon_id={IconId::BootstrapTrash} /> {" Remove Player"}
                        </Button>
                    </div>
                </div>

                // Champ selectors.
                // <ul class={"list-group list-group-flush"}>
                //     {
                //         champ_selections.iter().map(|(champ_id, lanes)| html!{
                //             <li class={"list-group-item"}>
                //                 <ChampSelection
                //                     change_champ_callback={change_champ_callback.clone()}
                //                     other_available_champs={other_available_champs.clone()}
                //                     lane_change_callback={lane_change_callback.clone()}
                //                     remove_champ_callback={remove_champ_callback.clone()}

                //                     selected_champ={
                //                         Some((champ_name.clone(), *lanes))
                //                     }
                //                 />
                //             </li>
                //         }).collect::<Html>()
                //     }
                //     <li class={"list-group-item"}>
                //         <ChampSelection
                //             selected_champ={None}
                //             change_champ_callback={change_champ_callback.clone()}
                //             other_available_champs={other_available_champs.clone()}
                //             // Leave the lane-change callback and remove champ callback no-ops
                //             // because there should not be any lanes or champ data
                //             // on an empty champ selector.
                //             lane_change_callback={Callback::noop()}
                //             remove_champ_callback={Callback::noop()}
                //         />
                //     </li>
                // </ul>
            </div>
        }
    }
}

/*
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
            }
}
*/

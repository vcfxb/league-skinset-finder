//! Player component.

use yew::{function_component, html, Callback, Html, Properties};

use super::app::{Players, PlayersAction};
use name_field::Name;
use super::button::Button;
use yew_icons::{Icon, IconId};

mod name_field;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub index: usize,
    pub players_list: Players,
    pub players_dispatch: Callback<PlayersAction>
}

#[function_component]
pub fn Player(props: &Props) -> Html {
    let players = props.players_list.clone();
    let player_name = players.0.borrow()[props.index].name.clone();
    
    let name_change = {
        let players_dispatch = props.players_dispatch.clone();
        let index = props.index;

        Callback::from(move |new_name| {
            players_dispatch.emit(PlayersAction::Rename { index, new_name })
        })
    };

    let remove_player = {
        let players_dispatch = props.players_dispatch.clone();
        let index = props.index;
        
        Callback::from(move |_| {
            players_dispatch.emit(PlayersAction::Delete(index))
        })
    };

    let disable_remove = props.players_list.len() <= 1;

    html! {
        <div class={"card mt-2 bg-light text-dark"}>
            <div class={"card-body row g-2 align-items-center w-100"}>
                <div class={"col-10"}>
                    // Name field and handling
                    <Name player_id={props.index} player_name={player_name} onchange={name_change} />
                </div>
                <div class={"col-2"}>
                    // Remove player button.
                    <Button disabled={disable_remove} on_click={remove_player} class={"btn btn-danger w-100 fs-5 py-2"} >
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

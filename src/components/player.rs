//! Player component.

use super::app::{Players, PlayersAction};
use super::button::Button;
use champ_select::ChampSelectCard;
use derive_more::derive::Display;
use name_field::Name;
use yew::{function_component, html, Callback, ContextProvider, Html, Properties};
use yew_icons::{Icon, IconId};

mod champ_dropdown;
mod champ_select;
mod name_field;

#[derive(Clone, Copy, Debug, PartialEq, Display)]
pub struct PlayerIndex(pub usize);

#[derive(Properties, PartialEq)]
pub struct Props {
    pub index: usize,
    pub players_list: Players,
    pub players_dispatch: Callback<PlayersAction>,
}

#[function_component]
pub fn Player(props: &Props) -> Html {
    let players = props.players_list.clone();
    let player_name = players.0.borrow()[props.index].name.clone();
    let champs_len = players.0.borrow()[props.index].champs.len();

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

        Callback::from(move |_| players_dispatch.emit(PlayersAction::Delete(index)))
    };

    let disable_remove = props.players_list.len() <= 1;

    html! {
        <ContextProvider<PlayerIndex> context={PlayerIndex(props.index)}>
            <ContextProvider<Players> context={props.players_list.clone()}>
                <ContextProvider<Callback<PlayersAction>> context={props.players_dispatch.clone()}>
                    <div class={"card mt-2 bg-light text-dark"}>
                        <div class={"card-body row g-2 align-items-center w-100"}>
                            <div class={"col-10"}>
                                // Name field and handling
                                <Name player_name={player_name} onchange={name_change} />
                            </div>
                            <div class={"col-2"}>
                                // Remove player button.
                                <Button disabled={disable_remove} on_click={remove_player} class={"btn btn-danger w-100 fs-5 py-2"} >
                                    <Icon icon_id={IconId::BootstrapTrash} /> {" Remove Player"}
                                </Button>
                            </div>
                        </div>

                        // Champ selectors.
                        <ul class={"list-group list-group-flush"}>
                            {
                                (0..champs_len)
                                    .map(|champ_index| html! {
                                        <li class={"list-group-item"}>
                                            <ChampSelectCard {champ_index} />
                                        </li>
                                    })
                                    .collect::<Html>()
                            }
                            <li class={"list-group-item"}>
                                <ChampSelectCard champ_index={champs_len} />
                            </li>
                        </ul>
                    </div>
                </ContextProvider<Callback<PlayersAction>>>
            </ContextProvider<Players>>
        </ContextProvider<PlayerIndex>>
    }
}

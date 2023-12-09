//! Yew components to build out the League Skinset Finder frontend. 

use std::{collections::HashSet, rc::Rc};
use enumflags2::BitFlags;
use yew::prelude::*;
use link::Link;
use crate::lanes::Lane;
use player::Player;
use yew_icons::{Icon, IconId};
use serde::{Serialize, Deserialize};
use skinset_list::SkinsetList;
use results_table::ResultsTable;

mod player;
mod link;
mod checkbox;
mod button;
mod results_table;
mod skinset_list;

/// State persisted for each player in the frontend. 
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PlayerRecord {
    /// Player name (optional -- resolve with player number otherwise).
    pub name: Option<AttrValue>,
    /// List of champs and what lanes for them. This is stored in an [`Rc`]'d [`RefCell`] for easy cloning/sharing
    /// with interior mutability. 
    pub champs: Rc<Vec<(AttrValue, BitFlags<Lane>)>>,
}

impl PlayerRecord {
    /// Create a new player with a given number and otherwise empty fields. 
    pub fn new() -> Self {
        Self { name: None, champs: Rc::new(Vec::with_capacity(170)) }
    }
}

/// Messages that can be passed to the top-level app.
pub enum Msg {
    /// A players name has been updated. 
    PlayerNameUpdate {
        index: usize,
        new_name: String,
    },

    /// Add a player on to the end of the list.
    AddPlayer,

    /// Remove a player from the list.
    RemovePlayer {
        /// The index of the player to remove. 
        player_index: usize,
    },

    /// When a player updates their champ list this component has to re-render. 
    PlayerChampListUpdate {
        player_id: usize,
        new_champ_list: Vec<(AttrValue, BitFlags<Lane>)>
    },

    /// When the list of skinsets is updated this component has to re-render. 
    SkinsetListUpdate {
        new_skinset_exclusion_list: HashSet<AttrValue>
    },
}

/// The main component that the frontend is rendered as. 
#[derive(Debug)]
pub struct App {
    /// The five players (max) in the league comp. 
    players: Vec<PlayerRecord>,
    /// The list of skins excluded from consideration. 
    skinsets_excluded: HashSet<AttrValue>,
}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        log::info!("App component created");
        // Create the list of players stored in this app.
        let mut players = Vec::with_capacity(5);
        // Add the default player.
        players.push(PlayerRecord::new());
        // Make default list of skinsets to exclude 
        let skinsets_excluded = ["N/A", "Legacy"].into_iter().map(AttrValue::from).collect();
        // Return
        App { players, skinsets_excluded: skinsets_excluded }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // Handle a player updating their name. 
            Msg::PlayerNameUpdate { index, new_name } => {
                // Update the player name in this object's model. 
                self.players[index].name = if new_name.is_empty() { None } else { Some(new_name.into()) };
            },

            Msg::AddPlayer => if self.players.len() <= 5 { self.players.push(PlayerRecord::new()) }

            Msg::RemovePlayer { player_index } => if self.players.len() >= 1 { self.players.remove(player_index); }

            Msg::SkinsetListUpdate { new_skinset_exclusion_list } => {
                // Replace the state and re-render.
                self.skinsets_excluded = new_skinset_exclusion_list;
            }

            Msg::PlayerChampListUpdate { player_id, new_champ_list } => {
                // Update appropriate champ list and re-render. 
                self.players[player_id].champs = Rc::new(new_champ_list);
            }
        }

        // 
        log::info!("Re-rendering page");
        // Always return true to indicate the need for a re-render.
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Resolve whether any players can be removed currently. 
        let enable_player_removal = self.players.len() > 1;

        html! {
            // Add a margin an padding to the bottom to force scroll bar to appear slightly sooner
            <div class={"pb-5 mb-5"}>
                <div class="mt-3 card bg-light text-dark"> 
                    <div class="card-body"> 
                        <p class="h1"> {"League of Legends skinset finder"} </p>
                        <p> {"This tool is used to find League of Legend team comps that share skins from the same skinset."} </p>
                        <p> 
                            {"I currently source my skin data from "} 
                            <Link href="https://leagueoflegends.fandom.com/wiki/Champion_skin/Skin_themes" open_in_new_tab={true} />
                            {", and my lane data from "}
                            <Link href="https://leagueoflegends.fandom.com/wiki/List_of_champions_by_draft_position" open_in_new_tab={true} />
                            {"."}
                        </p>
                        <p> {"Data was last updated from these sources on October 11th, 2023."} </p>
                        <p>
                            {"
                            I will try to keep this generally up to date with league skins and champions, but may not always
                            remember to update this every patch. If you notice that the date above is a long time ago, or there
                            are champs/skins missing, please let me know by filing an Issue report at 
                            "}
                            <Link href="https://github.com/Alfriadox/league-skinset-finder/issues" open_in_new_tab={true} /> {"."}
                        </p>
                    </div>
                </div>

                <SkinsetList 
                    excluded_skinsets={self.skinsets_excluded.clone()} 
                    
                    update_skinset_selection={ 
                        ctx.link().callback(move |new_value| Msg::SkinsetListUpdate { new_skinset_exclusion_list: new_value }) 
                    }
                />
                
                {
                    self.players.iter()
                        .enumerate()
                        .map(|(id, player)| {
                            html! { 
                                <Player 
                                    {id}
                                    name={player.name.clone()}
                                    champs={player.champs.clone()}

                                    on_name_change={
                                        ctx.link().callback(move |new_name| {
                                            Msg::PlayerNameUpdate { index: id, new_name }
                                        })
                                    }

                                    enable_remove={enable_player_removal}
                                    on_remove={
                                        ctx.link().callback(move |_| {
                                            Msg::RemovePlayer { player_index: id }
                                        })
                                    }

                                    on_champ_list_update={ ctx.link().callback(move |new_champ_list| Msg::PlayerChampListUpdate {
                                        player_id: id,
                                        new_champ_list
                                    }) }
                                /> 
                            }
                        })
                        .collect::<Html>()
                }

                // Block button to add a player. 
                <div class={"d-grid gap-2 my-2"}> 
                    <button 
                        type={"button"} 
                        class={"btn btn-success"}
                        disabled={self.players.len() == 5}

                        // On-click handler to add a player.
                        onclick={
                            ctx.link().callback(move |_| {
                                Msg::AddPlayer
                            })
                        }
                    >
                        <Icon icon_id={IconId::BootstrapPersonAdd} /> {" Add Player"}
                    </button>
                </div>

                // Table component to be rendered here. 
                <ResultsTable players={self.players.clone()} skinsets_excluded={self.skinsets_excluded.clone()} />
            </div>
        }
    }
}

//! The root App component that contains the entirety of the frontend. 
//! 

use crate::constants::{Lane, SkinsetId, ChampId};
use enumflags2::BitFlags;
use super::link::Link;
use super::button::Button;
// use super::player::Player;
// use super::results_table::ResultsTable;
use serde::{Deserialize, Serialize};
use super::skinset_list::SkinsetList;
use yew::prelude::*;
// use yew_icons::{Icon, IconId};

/// State persisted for each player in the frontend.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PlayerRecord {
    /// Player name (optional -- resolve with player number otherwise).
    pub name: Option<AttrValue>,

    /// List of champs and what lanes for them. This is in the order that they're in in the UI. 
    pub champs: Vec<(ChampId, BitFlags<Lane>)>,
}

impl PlayerRecord {
    /// Create a new player with a given number and otherwise empty fields.
    pub fn new() -> Self {
        Self {
            name: None,
            champs: Vec::with_capacity(170),
        }
    }

    /// Update the lanes for a champ for this player. 
    fn update_champ(&mut self, champ_id: ChampId, lanes: BitFlags<Lane>) {
        // Find the champ id to update if they're in this list already. 
        self.champs
            .iter()
            .enumerate()
            .find(|(_, (iter_champ_id, _))| *iter_champ_id == champ_id)
            .map(|(index, _)| index)
            // Once the index is found, update the lanes for the champ. 
            .map(|index| self.champs[index].1 = lanes);
    }

    /// Remove a champ on this player.
    fn remove_champ(&mut self, champ_id: ChampId) {
        // Remove the champ if they already were in the list. 
        let remove_index = self.champs
            .iter()
            .enumerate()
            .find(|(_, (iter_champ_id, _))| *iter_champ_id == champ_id)
            .map(|(index, _)| index);
        
        if let Some(index) = remove_index {
            // Just use remove here to maintain order.
            self.champs.remove(index);
        }
    }
}

/// Messages that can be passed to the top-level app.
#[derive(Debug)]
pub enum AppMsg {
    /// A players name has been updated.
    PlayerNameUpdate { index: usize, new_name: String },

    /// Add a player on to the end of the list.
    AddPlayer,

    /// Remove a player from the list.
    RemovePlayer {
        /// The index of the player to remove.
        player_index: usize,
    },

    /// Mark all skinsets as excluded. 
    ExcludeAllSkinsets,

    /// Mark all skinsets as included. 
    IncludeAllSkinsets,

    /// Toggle whether a skinset is excluded or included. 
    ToggleSkinset {
        /// The id of the skinset to toggle. 
        skinset_id: SkinsetId,
    },

    // /// Add or update a champ for a player. 
    // UpsertChampOnPlayer {
    //     /// The index of the player to update
    //     player_index: usize, 
    //     /// The champ ID to update.
    //     champ_id: ChampId,
    //     /// The new lanes for the champ (or default lanes). 
    //     lanes: BitFlags<Lane>
    // },

    // /// Remove a champion from a player. 
    // RemoveChampFromPlayer {
    //     /// The index of the player to update
    //     player_index: usize,
    //     /// The champ id to remove. 
    //     champ_id: ChampId,
    // },

    // /// Toggle the exclusion of a skinset by id. 
    // ToggleSkinsetExclusion {
    //     skinset_id: SkinsetId
    // }
}

/// The main component that the frontend is rendered as.
#[derive(Debug)]
pub struct App {
    /// The five players (max) in the league comp.
    pub players: Vec<PlayerRecord>,
    /// The list of skins excluded from consideration. This should stay sorted and deduplicated. 
    pub skinsets_excluded: Vec<SkinsetId>,
}

impl Component for App {
    type Message = AppMsg;

    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        log::info!("App component created");
        // Create the list of players stored in this app.
        let mut players = Vec::with_capacity(5);
        // Add the default player.
        players.push(PlayerRecord::new());

        // Return
        App {
            players,
            skinsets_excluded: Vec::from(SkinsetId::DEFAULT_EXCLUDED_SKINSETS),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // Handle a player updating their name.
            AppMsg::PlayerNameUpdate { index, new_name } => {
                // Update the player name in this object's model.
                self.players[index].name = if new_name.is_empty() {
                    None
                } else {
                    Some(new_name.into())
                };
            }

            AppMsg::AddPlayer => {
                if self.players.len() <= 5 {
                    self.players.push(PlayerRecord::new())
                }
            }

            AppMsg::RemovePlayer { player_index } => {
                if self.players.len() >= 1 {
                    self.players.remove(player_index);
                }
            }

            AppMsg::ExcludeAllSkinsets => {
                self.skinsets_excluded = SkinsetId::iter_all().collect();
            }

            AppMsg::IncludeAllSkinsets => {
                self.skinsets_excluded.clear();
            }

            AppMsg::ToggleSkinset { skinset_id } => {
                match self.skinsets_excluded.binary_search(&skinset_id) {
                    // If found, remove (maintaining order).
                    Ok(index) => { self.skinsets_excluded.remove(index); },
                    // If not found, insert (maintaining order).
                    Err(index) => self.skinsets_excluded.insert(index, skinset_id),
                }
            }

            // AppMsg::UpsertChampOnPlayer { player_index, champ_id, lanes } => {
            //     self.players[player_index].upsert_champ(champ_id, lanes);
            // }

            // AppMsg::RemoveChampFromPlayer { player_index, champ_id } => {

            // }
        }

        // Log that we're re-rendering the root application. 
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
                        <p> {"Data was last updated from these sources on December 8th, 2023."} </p>
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

                // Skinset list component will automatically reach up into this App through the dom tree and 
                // get a reference to the "global" skinset exclusion list. 
                <SkinsetList />

                // {
                //     self.players.iter()
                //         .enumerate()
                //         .map(|(id, player)| {
                //             html! {
                //                 <Player
                //                     {id}
                //                     name={player.name.clone()}
                //                     champs={player.champs.clone()}

                //                     on_name_change={
                //                         ctx.link().callback(move |new_name| {
                //                             Msg::PlayerNameUpdate { index: id, new_name }
                //                         })
                //                     }

                //                     enable_remove={enable_player_removal}
                //                     on_remove={
                //                         ctx.link().callback(move |_| {
                //                             Msg::RemovePlayer { player_index: id }
                //                         })
                //                     }

                //                     on_champ_list_update={ ctx.link().callback(move |new_champ_list| Msg::PlayerChampListUpdate {
                //                         player_id: id,
                //                         new_champ_list
                //                     }) }
                //                 />
                //             }
                //         })
                //         .collect::<Html>()
                // }

                // Block button to add a player.
                // <div class={"d-grid gap-2 my-2"}>
                //     <button
                //         type={"button"}
                //         class={"btn btn-success"}
                //         disabled={self.players.len() == 5}

                //         // On-click handler to add a player.
                //         onclick={
                //             ctx.link().callback(move |_| {
                //                 Msg::AddPlayer
                //             })
                //         }
                //     >
                //         <Icon icon_id={IconId::BootstrapPersonAdd} /> {" Add Player"}
                //     </button>
                // </div>

                // // Table component to be rendered here.
                // <ResultsTable players={self.players.clone()} skinsets_excluded={self.skinsets_excluded.clone()} />
            </div>
        }
    }
}


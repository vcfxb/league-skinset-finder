
use std::rc::Rc;
use std::{cell::RefCell, collections::HashSet};
use yew::{function_component, html, use_reducer, Callback, Html, Reducible};
use crate::{constants::SkinsetId, model::PlayerRecord};
use super::link::Link;
use super::skinset_list::SkinsetList;

pub struct Players(pub Rc<RefCell<Vec<PlayerRecord>>>);

pub enum PlayersAction {
    Create,
}

impl Reducible for Players {
    type Action = PlayersAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        // Take a mutable reference to internal state.
        let mut inner = self.0.borrow_mut();

        match action {
            // Create a new player if there are less than 5.
            PlayersAction::Create if inner.len() < 5 => inner.push(PlayerRecord::new()),
            PlayersAction::Create => log::warn!("Cannot create more than 5 players"),
        }

        // Drop the mutable reference and return self.
        drop(inner);
        self
    }
}

#[derive(PartialEq, Clone)]
pub struct IncludedSkinsets(pub Rc<RefCell<HashSet<SkinsetId>>>);

pub enum IncludedSkinsetsAction {
    ExcludeAll,
    IncludeAll,
    Toggle(SkinsetId)
}

impl Reducible for IncludedSkinsets {
    type Action = IncludedSkinsetsAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut inner = self.0.borrow_mut();

        match action {
            IncludedSkinsetsAction::ExcludeAll => inner.clear(),
            IncludedSkinsetsAction::IncludeAll => inner.extend(SkinsetId::iter_all()),

            IncludedSkinsetsAction::Toggle(skinset_id) => {
                if inner.contains(&skinset_id) {
                    inner.remove(&skinset_id); 
                } else {
                    inner.insert(skinset_id);
                }
            }
        }

        drop(inner);
        self
    }
}

#[function_component]
pub fn App() -> Html {
    let players = use_reducer(|| Players(Rc::new(RefCell::new(vec![PlayerRecord::new()]))));

    let skinsets = use_reducer(|| {
        IncludedSkinsets(Rc::new(RefCell::new(SkinsetId::generate_default_included_skinsets())))
    });

    let skinsets_dispatch = {
        let skinsets = skinsets.clone();

        Callback::from(move |action| {
            skinsets.dispatch(action)
        })
    };
    
    // Resolve whether any players can be removed currently.
    let enable_player_removal = players.0.borrow().len() > 1;

    html! {
        // Add a margin an padding to the bottom to force scroll bar to appear slightly sooner
        <div class="pb-5 mb-5">
            <>
                <div class="mt-3 card bg-light text-dark">
                    <div class="card-body">
                        <p class="h1"> {"League of Legends skinset finder"} </p>
                        <p> {"This tool is used to find League of Legend team comps that share skins from the same skinset."} </p>
                        <p>
                            {"I currently source my skin data from "}
                            <Link href="https://leagueoflegends.fandom.com/wiki/Champion_skin/Skin_themes" open_in_new_tab=true />
                            {", and my lane data from "}
                            <Link href="https://leagueoflegends.fandom.com/wiki/List_of_champions_by_draft_position" open_in_new_tab=true />
                            {"."}
                        </p>
                        <p> {"Data was last updated from these sources on December 8th, 2023."} </p>
                        <p>
                            {"
                            I will try to keep this generally up to date with league skins and champions, but may not always
                            remember to update this every patch. If you notice that the date above is a long time ago, or there
                            are champs/skins missing, please let me know by filing an Issue report at 
                            "}
                            <Link href="https://github.com/vcfxb/league-skinset-finder/issues" open_in_new_tab=true /> 
                            {"."}
                        </p>
                    </div>
                </div>

                <SkinsetList skinset_list={(*skinsets).clone()} change_skinset_list={skinsets_dispatch} />
            </>
        </div>
    }
}

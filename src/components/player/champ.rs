//! Champion component used for player champ lists. 

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChampSelectorProps {
    /// The selected champion. 
    pub selected_champ: AttrValue,
}

/// Champion selector for a player. 
#[function_component(ChampionSelector)]
pub fn champ_selector(props: &ChampSelectorProps) -> Html {
    unimplemented!()
}

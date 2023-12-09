//! Champion component used for player champ lists. 

use std::rc::Rc;
use enumflags2::BitFlags;
use yew::prelude::*;
use crate::lanes::Lane;
use crate::components::{
    player::champ_dropdown::ChampDropdown,
    button::Button
};
use super::lanes_select::LaneSelect;
use yew_icons::{Icon, IconId};

/// Champ Selection component including champion drop-down, lane checkboxes and button to remove champion from player. 
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChampSelection;

/// Properties passed to the champ selection component. 
#[derive(Properties, PartialEq)]
pub struct ChampSelectionProps {
    /// The selected champion and lanes. 
    pub selected_champ: Option<(AttrValue, BitFlags<Lane>)>,
    /// The list of other champions not currently selected by this player. 
    pub other_available_champs: Rc<Vec<AttrValue>>,
    /// Callback to handle changes to the champ selected -- passes the previously selected champ for ease of use. 
    pub change_champ_callback: Callback<(Option<AttrValue>, String)>,
    /// Callback to handle changes to the lanes selected. 
    pub lane_change_callback: Callback<(AttrValue, BitFlags<Lane>)>,
    /// Callback to remove this champ from the player. 
    pub remove_champ_callback: Callback<AttrValue>,
}

/// Messages passed to the champ selection component by its callbacks.
pub enum Msg {
    /// Change the selected champion. 
    ChangeChamp {
        new_champ: String
    },

    /// Change in the lanes for the selected champion.
    ChangeLanes {
        new_lanes: BitFlags<Lane>
    },

    /// Remove this champion from the player. 
    RemoveChamp
}


impl Component for ChampSelection {
    type Message = Msg;

    type Properties = ChampSelectionProps;

    fn create(_: &Context<Self>) -> Self {
        Self
    }
    
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangeChamp { new_champ } => {
                // Get the currently selected champ. 
                let currently_selected: Option<AttrValue> = ctx
                    .props()
                    .selected_champ
                    .as_ref()
                    .map(|(champ_name, _)| champ_name.clone());

                // Trigger the callback to the parent.
                ctx.props().change_champ_callback.emit((currently_selected, new_champ)); 
            }

            Msg::ChangeLanes { new_lanes } => {
                // Get the currently selected champ name. Use unwrap here as lane selection should only be available
                // on selected champs. 
                let current_champ_name: AttrValue = ctx.props().selected_champ.as_ref().unwrap().0.clone();

                // Trigger the callback to the parent. 
                ctx.props().lane_change_callback.emit((current_champ_name, new_lanes)); 
            }

            Msg::RemoveChamp => {
                // Get the currently selected champ name. Use unwrap here as lane selection should only be available
                // on selected champs. 
                let current_champ_name: AttrValue = ctx.props().selected_champ.as_ref().unwrap().0.clone();
                
                // Trigger the callback to the parent. 
                ctx.props().remove_champ_callback.emit(current_champ_name); 
            }
        }

        // Never cause a direct re-render of this object. 
        // Always pass to the parent object to re-render. 
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Get the properties passed to the component. 
        let ChampSelectionProps { 
            selected_champ, 
            other_available_champs, 
            ..
        } = ctx.props();

        // Get the selected champ name.
        let selected_champ_name = selected_champ.as_ref().map(|(champ_name, _)| champ_name.clone());

        html! {
            <div class={"card-body"}>
                <ChampDropdown 
                    selected_champ={selected_champ_name.clone()} 
                    other_available_champs={other_available_champs.clone()}
                    on_change={ ctx.link().callback(|new_champ| Msg::ChangeChamp { new_champ }) }
                />
                
                if selected_champ.is_some() {
                    <div class={"my-1 align-items-center"}>
                        <LaneSelect 
                            lanes={selected_champ.clone().unwrap().1}
                            update_lanes_callback={ ctx.link().callback(|new_lanes| Msg::ChangeLanes { new_lanes }) }
                        />
                    </div>
    
                    <Button class={"btn btn-danger w-100"} enable={true} on_click={ ctx.link().callback(|_| Msg::RemoveChamp) }>
                        <Icon icon_id={IconId::BootstrapTrash} /> {" Remove "} {selected_champ_name.clone()}
                    </Button>
                }
            </div>
        }
    }
}

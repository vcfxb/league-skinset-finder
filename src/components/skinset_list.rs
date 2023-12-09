//! Component/card listing all the skinsets to be played. 

use std::collections::HashSet;
use uuid::Uuid;
use yew::prelude::*;
use crate::skinsets::{GLOBAL_SKINSETS_MAP, Skinsets};
use yew_icons::{Icon, IconId};
use crate::components::button::Button;

/// Component storing a the exclusion/inclusion state of all the skinsets. 
pub struct SkinsetList {
    /// Is the body collapsed/hidden?
    collapsed: bool,
}

/// Messages passed to the SkinsetList component. 
pub enum Msg {
    /// Toggle whether the body is visible or hidden. 
    ToggleViewHide,

    /// Mark all skinsets as selected.
    SelectAllSkinsets,

    /// Mark all skinsets as excluded. 
    ExcludeAllSkinsets,

    /// Toggle the exclusion of a specific skinset. 
    ToggleSkinset { skinset: AttrValue }
}

#[derive(PartialEq, Properties)]
pub struct SkinsetListProps {
    /// List of currently excluded skinsets.
    pub excluded_skinsets: HashSet<AttrValue>,
    /// Callback that gets emitted when the skinset section needs to be re-rendered. 
    pub update_skinset_selection: Callback<HashSet<AttrValue>>,
}

impl Component for SkinsetList {
    type Message = Msg;

    type Properties = SkinsetListProps;

    fn create(_: &Context<Self>) -> Self {
        SkinsetList { collapsed: false }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleViewHide => {
                self.collapsed = !self.collapsed;
                // Re-render
                true
            }

            Msg::ExcludeAllSkinsets => {
                // Make a new set that excludes all skins and emit it back to the parent. 
                ctx.props()
                    .update_skinset_selection
                    .emit(GLOBAL_SKINSETS_MAP.with(Skinsets::all_skinsets));

                // This component does not re-render post parent 
                false
            }

            Msg::SelectAllSkinsets => {
                // Emit an empty set back to the parent. 
                ctx.props()
                    .update_skinset_selection
                    .emit(HashSet::new());

                // Do not re-render this component after the parent has re-rendered.
                false
            }

            Msg::ToggleSkinset { skinset } => {
                // Get the set of excluded skin sets.
                let mut excluded_skinsets = ctx.props().excluded_skinsets.clone();

                // Toggle the skinset in the hash set.
                if excluded_skinsets.contains(&skinset) {
                    excluded_skinsets.remove(&skinset);
                } else {
                    excluded_skinsets.insert(skinset);
                }

                // Send the updated version back to the parent. 
                ctx.props().update_skinset_selection.emit(excluded_skinsets);

                // Do not re-render this component separately. 
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Resolve the Icon to use for the show/hide button. 
        let collapse_button_icon: IconId = self.collapsed
            .then_some(IconId::HeroiconsOutlineEyeSlash)
            .unwrap_or(IconId::HeroiconsOutlineEye);

        // Make a node id for the collapsing div. 
        let collapse_id: AttrValue = Uuid::new_v4().to_string().into();

        html! {
            <div class="card bg-light text-dark my-2">
                <div class="card-body">
                    <span class="card-title d-inline-flex w-100"> 
                        <h3 class="p2 flex-grow-1"> 
                            {"Selected Skinsets"} 
                        </h3> 
    
                        // De-select all skinsets
                        <Button class={"btn btn-primary mx-1"} enable={!self.collapsed} on_click={ ctx.link().callback(|_| Msg::ExcludeAllSkinsets) }>
                            {"De-select All Skinsets"}
                        </Button>
    
                        // Select all button
                        <Button class={"btn btn-primary mx-1"} enable={!self.collapsed} on_click={ ctx.link().callback(|_| Msg::SelectAllSkinsets) }>
                            {"Select All Skinsets"}
                        </Button>
    
                        // Show/hide button
                        <Button class={"btn btn-secondary mx-1"} enable={true} on_click={ ctx.link().callback(|_| Msg::ToggleViewHide) }>
                            {"Show/Hide "} <Icon icon_id={collapse_button_icon} />
                        </Button>
                    </span>
                </div>
    
                // Collapsable body. 
                if !self.collapsed {
                    <div class="card-body row row-cols-6" id={collapse_id.clone()}>
                        {{
                            // Get excluded skinsets.
                            let excluded_skinsets = ctx.props().excluded_skinsets.clone();
                            
                            // Get list of all skinsets.
                            let mut all_skinsets: Vec<AttrValue> = GLOBAL_SKINSETS_MAP
                                .with(Skinsets::all_skinsets)
                                .into_iter()
                                .collect();
                            
                            // Sort alphabetically 
                            all_skinsets.sort();

                            // Iterate and generate html checkboxes and labels
                            all_skinsets
                                .iter()
                                .map(|skinset: &AttrValue| {
                                    // Determine wether this skinset is checked. 
                                    let checked = !excluded_skinsets.contains(skinset);
                                    // Make an ID for the checkbox.
                                    let checkbox_id: AttrValue = Uuid::new_v4().to_string().into();
                                    // Make a clone of the skinset to put in the callback.
                                    let skinset_clone = skinset.clone();
                                    // Make the on-change callback
                                    let onchange = ctx.link().callback(move |_| Msg::ToggleSkinset { skinset: skinset_clone.clone() });

                                    // Make a transformed skinset name to handle long skinset names.
                                    let transformed_skinset_name = if skinset.len() > 22 {
                                        format!("{}...", &skinset[0..21])
                                    } else {
                                        skinset.to_string()
                                    };

                                    html! {
                                        <div class={"col form-check"}>
                                            <input 
                                                class="form-check-input" 
                                                type="checkbox" 
                                                id={checkbox_id.clone()} 
                                                {onchange}
                                                {checked}
                                            />
                                            <label class="form-check-label" for={checkbox_id}>
                                                {transformed_skinset_name}
                                            </label>
                                        </div>
                                    }
                                })
                                .collect::<Html>()
                        }}
                    </div>
                }
            </div>
        }    
    }
}

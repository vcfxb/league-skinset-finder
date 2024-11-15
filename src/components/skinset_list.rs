//! Component/card listing all the skinsets to be played.

use yew::{function_component, use_state, Callback, Html, Properties, html};
use crate::constants::SkinsetId;

use super::app::{IncludedSkinsets, IncludedSkinsetsAction};
use super::button::Button;
use yew_icons::{Icon, IconId::*};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub skinset_list: IncludedSkinsets,
    pub change_skinset_list: Callback<IncludedSkinsetsAction>
}

#[function_component]
pub fn SkinsetList(props: &Props) -> Html {
    let collapsed = use_state(|| false);
    let cb = props.change_skinset_list.clone();

    let exclude_all_skinsets = {
        let cb = cb.clone();

        Callback::from(move |_| {
            cb.emit(IncludedSkinsetsAction::ExcludeAll)
        })
    };

    let include_all_skinsets = {
        let cb = cb.clone();

        Callback::from(move |_| {
            cb.emit(IncludedSkinsetsAction::IncludeAll)
        })
    };

    let toggle_vis = {
        let collapsed = collapsed.clone();
      
        Callback::from(move |_| {
            collapsed.set(!*collapsed)
        })
    };

    html! {
        <div class="card bg-light text-dark my-2">
            <div class="card-body">
                <span class="card-title d-inline-flex w-100">
                    <h3 class="p2 flex-grow-1">
                        {"Selected Skinsets"}
                    </h3>

                    // De-select all skinsets
                    <Button class="btn btn-primary mx-1" disabled={*collapsed} on_click={exclude_all_skinsets}>
                        {"De-select All Skinsets"}
                    </Button>

                    // Select all button
                    <Button class="btn btn-primary mx-1" disabled={*collapsed} on_click={include_all_skinsets}>
                        {"Select All Skinsets"}
                    </Button>

                    // Show/hide button
                    <Button class="btn btn-secondary mx-1" on_click={toggle_vis}>
                        if *collapsed {
                            {"Show "} <Icon icon_id={BootstrapEyeFill} />
                        } else {
                            {"Hide "} <Icon icon_id={BootstrapEyeSlashFill} />
                        }
                    </Button>
                </span>
            </div>
            
            if !*collapsed {
                <div class="card-body row row-cols-6">
                    {
                        SkinsetId::iter_all()
                            .map(|skinset_id: SkinsetId| {
                                // Make a transformed skinset name to handle long skinset names.
                                let transformed_skinset_name = if skinset_id.skinset_name().len() > 22 {
                                    format!("{}...", &skinset_id.skinset_name()[0..21])
                                } else {
                                    skinset_id.skinset_name().to_string()
                                };
                                
                                // Make an ID for the checkbox.
                                let checkbox_id: String = format!("skinset-{}-check", skinset_id.inner());

                                let onchange = {
                                    let cb = cb.clone();

                                    Callback::from(move |_| {
                                        cb.emit(IncludedSkinsetsAction::Toggle(skinset_id))
                                    })
                                };

                                let checked = (*props.skinset_list.0).borrow().contains(&skinset_id);

                                html! {
                                    <div class="col form-check">
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
                    }
                </div>
            }
        </div>
    }
}

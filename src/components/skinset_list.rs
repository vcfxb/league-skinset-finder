//! Component/card listing all the skinsets to be played.

use crate::{components::button::Button, constants::SkinsetId};
use uuid::Uuid;
use yew::{prelude::*, html::Scope};
use yew_icons::{Icon, IconId};
use super::{App, app::AppMsg};

/// Component storing a the exclusion/inclusion state of all the skinsets.
pub struct SkinsetList {
    /// Is the body collapsed/hidden?
    collapsed: bool,
}

impl SkinsetList {
    /// Get the parent app component from self. 
    fn get_parent_app_scope(ctx: &Context<Self>) -> Scope<App> {
        ctx.link()
            .get_parent()
            .expect("parent component exists")
            .downcast()
    }

    // fn get_parent_app(ctx: &Context<Self>) -> impl Deref<Target = App> {
    //     SkinsetList::get_parent_app_scope(ctx)
    //         .get_component()
    //         .expect("got parent App component")
    // }
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
    ToggleSkinset { skinset_id: SkinsetId },
}

impl Component for SkinsetList {
    type Message = Msg;

    type Properties = ();

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
                // Bubble message to parent app component. 
                SkinsetList::get_parent_app_scope(ctx).send_message(AppMsg::ExcludeAllSkinsets);
                // This must be re-render after sending the message to the parent (as there is no new props).
                true
            }

            Msg::SelectAllSkinsets => {
                // Bubble up the message to the parent. 
                SkinsetList::get_parent_app_scope(ctx).send_message(AppMsg::IncludeAllSkinsets);
                // This must be re-render after sending the message to the parent (as there is no new props).
                true
            }

            Msg::ToggleSkinset { skinset_id } => {
                // Bubble message to parent.
                SkinsetList::get_parent_app_scope(ctx).send_message(AppMsg::ToggleSkinset { skinset_id });
                // Do not re-render this component separately.
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Resolve the Icon to use for the show/hide button.
        let collapse_button_icon: IconId = self
            .collapsed
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
                            // Get the parent app scope. 
                            let parent_app_scope = SkinsetList::get_parent_app_scope(ctx);
                            // Get the parent app itself (as a ref). 
                            let parent_app = parent_app_scope
                                .get_component()
                                .expect("got parent App Component");

                            // Get a reference to the list of excluded skinsets from the parent component. 
                            let excluded_skinsets: &[SkinsetId] = parent_app.skinsets_excluded.as_slice();

                            // Get an iterator over all skinset ids in alphabetical order by name. 
                            let all_skinsets = SkinsetId::iter_all();

                            // Iterate and generate html checkboxes and labels
                            all_skinsets
                                .map(|skinset_id: SkinsetId| {
                                    // Determine wether this skinset is checked.
                                    let checked = !excluded_skinsets.contains(&skinset_id);
                                    // Make an ID for the checkbox.
                                    let checkbox_id: AttrValue = Uuid::new_v4().to_string().into();
                                    // Make the on-change callback.
                                    let onchange = ctx.link().callback(move |_| Msg::ToggleSkinset { skinset_id });

                                    // Make a transformed skinset name to handle long skinset names.
                                    let transformed_skinset_name = if skinset_id.skinset_name().len() > 22 {
                                        format!("{}...", &skinset_id.skinset_name()[0..21])
                                    } else {
                                        skinset_id.skinset_name().to_string()
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

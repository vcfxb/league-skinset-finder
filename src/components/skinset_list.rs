//! Component/card listing all the skinsets to be played.

use crate::{components::button::Button, constants::SkinsetId};
use icondata::{BiHideRegular, BiShowRegular};
use leptos::{
    component, create_effect, create_memo, create_node_ref, create_rw_signal, ev::Event,
    event_target_checked, html::Input, view, CollectView, For, Fragment, IntoView, Memo, NodeRef,
    RwSignal, Show, Signal, SignalGet, SignalSet, SignalUpdate, View,
};
use leptos_icons::Icon;
use std::collections::HashSet;

/// An interactive card that displays all the skinsets available with information on which ones are selected.
///
/// # Arguments
/// - `skinsets_rw_signal` - A read/write signal to the current [`HashSet`] of skinsets selected.
#[component]
pub fn SkinsetList(skinsets_rw_signal: RwSignal<HashSet<SkinsetId>>) -> impl IntoView {
    // Create a signal to track the state of whether the card is collapsed.
    let collapsed: RwSignal<bool> = create_rw_signal(false);

    // Derive a signal to track the icon to use for the show/hide button.
    let show_hide_body: Signal<Fragment> = Signal::derive(move || {
        if collapsed.get() {
            view! { "Show " <Icon icon=BiShowRegular /> }
        } else {
            view! { "Hide " <Icon icon=BiHideRegular /> }
        }
    });

    // Closure to exclude all skinsets.
    let exclude_all_skinsets = move |_| {
        log::debug!("Excluding all skinsets");
        skinsets_rw_signal.update(|hash_set: &mut HashSet<SkinsetId>| hash_set.clear())
    };

    // Closure to include all skinsets.
    let include_all_skinsets = move |_| {
        log::debug!("Including all skinsets");
        skinsets_rw_signal.set(SkinsetId::iter_all().collect())
    };

    // Create a derived that will produce the rendered list of skinsets with checkboxes.
    let skinset_checkboxes: Memo<View> = create_memo(move |_| {
        // For each skinset that exists, determine a good shortened name and render a view.
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

                // Make a checkbox node ref so that we can manually debounce checkbox events since stuff seems broken.
                let node_ref: NodeRef<Input> = create_node_ref::<Input>();

                // Make a function to handle the checkbox.
                let on_change = move |ev: Event| {
                    if event_target_checked(&ev) {
                        skinsets_rw_signal.update(|hash_set: &mut HashSet<SkinsetId>| {
                            hash_set.insert(skinset_id);
                        });
                    } else {
                        skinsets_rw_signal.update(|hash_set: &mut HashSet<SkinsetId>| {
                            hash_set.remove(&skinset_id);
                        });
                    }
                };

                // Use an effect to update the state of the checkbox in the browser when it's clicked.
                // This is necessary to prevent checkboxes from misbehaving.
                create_effect(move |_| {
                    node_ref
                        .get_untracked()
                        .unwrap()
                        .set_checked(skinsets_rw_signal.get().contains(&skinset_id))
                });

                view! {
                    <div class="col form-check">
                        <input
                            _ref=node_ref
                            class="form-check-input"
                            type="checkbox"
                            id={checkbox_id.clone()}
                            on:change=on_change
                            checked={move || skinsets_rw_signal.get().contains(&skinset_id)}
                        />
                        <label class="form-check-label" for={checkbox_id}>
                            {transformed_skinset_name}
                        </label>
                    </div>
                }
            })
            .collect_view()
    });

    view! {
        <div class="card bg-light text-dark my-2">
            <div class="card-body">
                <span class="card-title d-inline-flex w-100">
                    <h3 class="p2 flex-grow-1">
                        "Selected Skinsets"
                    </h3>

                    // De-select all skinsets
                    <Button class="btn btn-primary mx-1" disabled={collapsed} on_click=exclude_all_skinsets>
                        "De-select All Skinsets"
                    </Button>

                    // Select all button
                    <Button class="btn btn-primary mx-1" disabled={collapsed} on_click=include_all_skinsets>
                        "Select All Skinsets"
                    </Button>

                    // Show/hide button
                    <Button class="btn btn-secondary mx-1" on_click={ move |_| collapsed.update(|c| *c = !*c) }>
                        {show_hide_body}
                    </Button>
                </span>
            </div>

            // Show/hide functionality -- show nothing when collapsed.
            <Show when={move || !collapsed.get()} fallback={move || view! {} }>
                <div class="card-body row row-cols-6">
                    {skinset_checkboxes}
                </div>
            </Show>
        </div>
    }
}

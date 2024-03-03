//! Checkbox components.

use leptos::{component, view, Callback, IntoView, MaybeSignal, Callable};

/// Checkbox component.
/// 
/// # Arguments
/// - `on_change` - [Callback] to
#[component]
pub fn Checkbox(
    #[prop(into)]
    checked: MaybeSignal<bool>,
    #[prop(into)]
    on_change: Callback<()>
) -> impl IntoView {

    view! {
        <input
            class="form-check-input"
            type="checkbox"
            checked={checked}
            on:change=move |_| on_change.call(())
        />
    }
}

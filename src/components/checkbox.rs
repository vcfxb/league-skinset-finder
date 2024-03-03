//! Checkbox components.

use leptos::{component, view, Callback, IntoView, MaybeSignal, Callable};

/// Checkbox component.
/// 
/// # Arguments
/// - `on_change` - [Callback] to
/// # WARNING
/// The reactivity of this component is likely buggy -- steal from the skinset selector when you get a chance. 
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

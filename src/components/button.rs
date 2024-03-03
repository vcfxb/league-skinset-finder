//! Clickable button component.

use leptos::{component, view, Callable, Callback, Children, IntoView, MaybeSignal};

/// A clickable button component. 
/// 
/// # Arguments
/// - `disabled` - Is this button disabled/unclickable? (default: false). This can be a signal. 
/// - `class` - The HTML class(es) used to style this button.
/// - `on_click` - The callback that is triggered when the button is pressed. 
/// - `children` - The children of this component that are rendered inside of it. 
#[component]
pub fn Button(
    #[prop(into, optional)]
    disabled: MaybeSignal<bool>,
    #[prop(into)]
    class: String,
    #[prop(into)]
    on_click: Callback<()>,
    children: Children
) -> impl IntoView {
    // We have to make a closure on stable for some reason. Use it as an opportunity to ignore the mouse event.
    let on_click_closure = move |_| { on_click.call(()) };

    view! {
        <button type="button" class=class disabled=disabled on:click=on_click_closure>
            {children()}
        </button>
    }
}

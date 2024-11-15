//! Clickable button component.

use yew::{function_component, html, AttrValue, Callback, Html, MouseEvent, Properties};

#[derive(PartialEq, Properties, Debug)]
pub struct Props {
    #[prop_or(false)]
    pub disabled: bool,
    pub class: AttrValue,
    pub on_click: Callback<()>,
    pub children: Html
}

/// A clickable button component. 
/// 
/// # Arguments
/// - `disabled` - Is this button disabled/unclickable? (default: false).
/// - `class` - The HTML class(es) used to style this button.
/// - `on_click` - The callback that is triggered when the button is pressed. 
/// - `children` - The children of this component that are rendered inside of it. 
#[function_component]
pub fn Button(props: &Props) -> Html {
    let cb = Callback::clone(&props.on_click);
    let onclick = move |_: MouseEvent| { cb.emit(()); };

    html! {
        <button type="button" class={&props.class} disabled={props.disabled} {onclick}>
            {props.children.clone()}
        </button>
    }
}

//! Checkbox components.

use yew::{function_component, html, Callback, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props {
    checked: bool,
    on_change: Callback<()>
}

#[function_component]
pub fn Checkbox(props: &Props) -> Html {
    let cb = {
        let on_change = props.on_change.clone();

        Callback::from(move |_| {
            on_change.emit(())
        })
    };

    html! {
        <input
            class="form-check-input"
            type="checkbox"
            checked={props.checked}
            onchange={cb}
        />
    }
}

//! Checkbox components.

use yew::prelude::*;

/// Properties passed to checkboxes.
#[derive(Properties, PartialEq)]
pub struct CheckboxProps {
    /// The id of the checkbox.
    pub id: Option<AttrValue>,
    /// Whether the checkbox is checked by default.
    pub checked: bool,
    /// Callback that is emitted when the checkbox is toggled -- includes the current/updated state of the checkbox.
    pub on_change: Callback<bool>,
}

/// Checkbox component.
#[function_component(Checkbox)]
pub fn checkbox(props: &CheckboxProps) -> Html {
    // Create callback to trigger when changed.
    let onchange = {
        // Clone the callback (cheap Rc clone).
        let callback = props.on_change.clone();
        // Get the current state of the checkbox.
        let checked = props.checked;

        // Make an event handler that uses state and emits a normalized callback.
        Callback::from(move |_| {
            // Emit a callback with the new/next state.
            callback.emit(!checked);
        })
    };

    html! {
        <input
            class={"form-check-input"}
            type={"checkbox"}
            id={props.id.clone()}
            checked={props.checked}
            {onchange}
        />
    }
}

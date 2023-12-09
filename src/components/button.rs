//! Clickable button component. 

use yew::prelude::*;

/// Properties passed to the button when rendering. 
#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    /// Is this button currently enabled/clickable?
    pub enable: bool,
    /// Callback that gets emitted when the button is clicked. 
    pub on_click: Callback<()>,
    /// The html classes used to style this button.
    pub class: AttrValue,
    /// The button content (passed as children of this component). 
    pub children: Html,
}

/// A button with arbitrary styling and content.
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {    
    // Make the closure to recieve button clicks.
    let click_handler = {
        // Clone the callback to trigger. 
        let callback = props.on_click.clone();
        // Make another callback that takes and ignores an event.
        Callback::from(move |_| callback.emit(()))
    };

    html! {
        <button 
            type={"button"} 
            class={ props.class.clone() }
            // Conditionally drop the handler for a no-op when disabled. 
            onclick={ if props.enable {Some(click_handler)} else { None } } 
            disabled={!props.enable} 
        >
            { props.children.clone() }
        </button>
    }
}

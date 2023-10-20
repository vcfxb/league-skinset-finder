//! Button component used to remove players. 

use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
pub struct RemoveProps {
    /// Is player removal enabled at this time?
    pub enable: bool,
    /// Callback to handle the player removal button being clicked.
    pub on_click: Callback<()>,
}

/// Component used to recieve player removals.
#[function_component(RemoveButton)]
pub fn remove_button(props: &RemoveProps) -> Html {
    
    // Make the closure to recieve button clicks.
    let on_click = {
        let callback = props.on_click.clone();
        move |_| callback.emit(())
    };

    
    html! { 
        <button 
            type={"button"} 
            class={"btn btn-danger w-100 fs-5 py-2"} 
            // Conditionally drop the handler for a no-op when disabled. 
            onclick={ if props.enable {Some(on_click)} else { None } } 
            disabled={!props.enable} 
        >
            <Icon icon_id={IconId::BootstrapTrash} /> {" Remove Player"} 
        </button>
    }
}

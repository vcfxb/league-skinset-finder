//! Player components in the league skinset finder. 

use yew::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct PlayerProps {
    /// Player number 1-5
    pub player_number: u8,
    /// Callback to handle player name chane. 
    pub on_name_change: Callback<String>
}


#[function_component(Player)]
pub fn player(props: &PlayerProps) -> Html {
    // Use a node ref to track changes to the input. 
    let input_node_ref = use_node_ref();

    // Format the player name text element id. 
    let name_id = format!("player-{}-name", props.player_number);

    // Make a callback to handle events.
    let on_name_input = {
        // Clone the input node ref
        let input_node_ref = input_node_ref.clone();
        // Clone the callback prop to invoke. 
        let pass_to = props.on_name_change.clone();

        // Make a callback that handles input events.
        Callback::from(move |_| {
            if let Some(input) = input_node_ref.cast::<HtmlInputElement>() {
                pass_to.emit(input.value());
            }
        })
    };

    html! {
        <div class={"card p-4 mt-2 bg-light text-dark"}>
            <div class={"form-floating"}> 
                <input ref={input_node_ref} id={name_id.clone()} type={"text"} class={"form-control"} placeholder={"First Last"} oninput={on_name_input} />
                <label for={name_id} > {"Player "} {props.player_number} {" Name"} </label>
            </div>
        </div>
    }
}


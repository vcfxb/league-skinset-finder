//! Component definition for the player's name field.

use uuid::Uuid;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NameProps {
    /// The index of this player in the list of players.
    pub player_id: usize,
    /// The default player name.
    pub player_name: AttrValue,
    /// The callback to emit when the name is changed.
    pub onchange: Callback<String>,
}

/// A component for rendering the player name field of each player in the skinset finder.
#[function_component(Name)]
pub fn name_field(props: &NameProps) -> Html {
    // Use the input node's ref to get the value of the text box whenever it's changed.
    let input_node_ref = use_node_ref();
    // Use uuid to make an ID that connects the two fields of the form and is reasonably expected to be globally unique.
    let id: AttrValue = Uuid::new_v4().to_string().into();

    // Make a callback to handle events.
    let on_name_input = {
        // Clone the input node ref
        let input_node_ref = input_node_ref.clone();
        // Clone the callback prop to invoke.
        let pass_to = props.onchange.clone();

        // Make a callback that handles input events.
        Callback::from(move |_| {
            if let Some(input) = input_node_ref.cast::<HtmlInputElement>() {
                pass_to.emit(input.value());
            }
        })
    };

    html! {
        <div class={"form-floating"}>
            <input
                ref={input_node_ref}
                id={id.clone()}
                type={"text"}
                class={"form-control"}
                placeholder={"First Last"}
                oninput={on_name_input}
                value={props.player_name.clone()}
            />

            <label for={id} > {"Player "} {props.player_id + 1} {" Name"} </label>
        </div>
    }
}

//! Champion dropdown component.

use std::rc::Rc;
use uuid::Uuid;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use crate::constants::ChampId;

/// Properties passed to the champion dropdown component.
#[derive(Properties, PartialEq)]
pub struct ChampDropdownProps {
    /// The selected champ, if there is one.
    pub selected_champ: Option<ChampId>,
    /// Shared list of all other available champs to select from
    pub other_available_champs: Rc<[ChampId]>,
    /// Callback emitted when a new champ is selected.
    pub on_change: Callback<ChampId>,
}

/// Dropdown component to choose a champ from.
#[derive(Debug)]
pub struct ChampDropdown {
    /// Node ref used to identify the select element.
    select_ref: NodeRef,
}

impl Component for ChampDropdown {
    type Message = ();

    type Properties = ChampDropdownProps;

    fn create(_: &Context<Self>) -> Self {
        ChampDropdown {
            select_ref: NodeRef::default(),
        }
    }

    // Custom `changed` implementation needed to deal with stupid select/auto-fill issue on firefox.
    fn changed(&mut self, ctx: &Context<Self>, _: &Self::Properties) -> bool {
        if let Some(select) = self.select_ref.cast::<HtmlSelectElement>() {
            if let Some(selected_champ) = ctx.props().selected_champ {
                select.set_value(selected_champ.champ_name());
            } else {
                select.set_value("Select a champion...");
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Get a reference to the properties passed to this component.
        let props = ctx.props();
        // Make a node ID to connect the select to the label.
        let select_id: AttrValue = Uuid::new_v4().to_string().into();

        // Make the event handler callback to be triggered when a champ is selected.
        let onchange = {
            // Clone the callback to pass the selected champ to.
            let callback = props.on_change.clone();
            // Clone the reference to the select node that we're waiting for a change on.
            let select_node_ref = self.select_ref.clone();

            Callback::from(move |_| {
                if let Some(select) = select_node_ref.cast::<HtmlSelectElement>() {
                    callback.emit(select.value());
                }
            })
        };

        // Make a list of all the champs and collect to a vec to keep as alphabetical.
        let mut all_listed_champs_alphabetical: Vec<&AttrValue> = props
            .other_available_champs
            .iter()
            .chain(props.selected_champ.iter())
            .collect();

        all_listed_champs_alphabetical.sort();

        html! {
            // Form tag in here is necessary to prevent firefox from auto selecting an option
            <div class="form-floating">
                <select ref={self.select_ref.clone()} id={select_id.clone()} class={"form-select"} aria-label={"Champion Selection"} {onchange} autocomplete="off">
                    // If there is no selected champs
                    if props.selected_champ.is_none() {
                        <option selected={true} disabled={true}>
                            {"Select a champion..."}
                        </option>
                    }

                    // All champs in list (handle for selections)
                    {
                        all_listed_champs_alphabetical
                            .iter()
                            .map(|champ_name| {
                                html! {
                                    <option selected={Some(champ_name) == props.selected_champ.clone().as_ref().as_ref()}>
                                        {champ_name}
                                    </option>
                                }
                            })
                            .collect::<Html>()
                    }

                </select>

                <label for={select_id}> {"Select a champion..."} </label>
            </div>
        }
    }
}

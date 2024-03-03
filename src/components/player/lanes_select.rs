//! Lane selector component attached to champ selector.

use crate::components::checkbox::Checkbox;
use crate::constants::Lane;
use enumflags2::BitFlags;
use uuid::Uuid;
use yew::prelude::*;

/// Properties passed to the lane selection component.
#[derive(PartialEq, Properties)]
pub struct LaneSelectProps {
    /// The selected lanes.
    pub lanes: BitFlags<Lane>,
    /// The callback emitted when the selected lanes are updated.
    pub update_lanes_callback: Callback<BitFlags<Lane>>,
}

/// Lane selectionc component attached to each champ.
#[function_component(LaneSelect)]
pub fn lanes_select(props: &LaneSelectProps) -> Html {
    // Make an iterator that generates a unique checkbox id for each lane/checkbox.
    let check_ids = BitFlags::<Lane>::all()
        .iter()
        .map(|_| AttrValue::from(Uuid::new_v4().to_string()));

    // Make iterator that generates a callback for each lane's checkbox.
    let callbacks = BitFlags::<Lane>::all().iter().map(|lane| {
        // Clone the passed callback (cheap Rc clone).
        let callback = props.update_lanes_callback.clone();
        // Copy the passed lanes from the props.
        let lanes = props.lanes;

        Callback::from(move |checked: bool| {
            // Move the lanes from the outer scope into this callback.
            let mut lanes = lanes;
            // Toggle the lane that this checkbox coresponds to.
            lanes.set(lane, checked);
            // Emit the updated lanes to the callback.
            callback.emit(lanes);
        })
    });

    // Make an iterator with each lane, id, and callback.
    let zipped_iter = BitFlags::<Lane>::all()
        .iter()
        .zip(check_ids)
        .zip(callbacks)
        // Transform the nested tuple int a flat 3-tuple.
        .map(|((lane, check_id), callback)| (lane, check_id, callback));

    html! {
        <> {
            zipped_iter.map(|(lane, check_id, callback)| html! {
                <div class={"form-check form-check-inline"}>
                    <Checkbox checked={props.lanes.contains(lane)} id={check_id.clone()} on_change={callback} />
                    <label class="form-check-label" for={check_id}> {lane.to_string()} </label>
                </div>
            }).collect::<Html>()
        } </>
    }
}

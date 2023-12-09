/// Link components in the league skinset finder.
use yew::prelude::*;
use yew_icons::{Icon, IconId};

/// Properties passed to link components.
#[derive(Properties, PartialEq)]
pub struct LinkProps {
    /// Open this link in a new tab?
    #[prop_or_default]
    pub open_in_new_tab: bool,
    /// The address being linked to.
    pub href: AttrValue,
    /// The text of the link (default to the address if none).
    #[prop_or_default]
    pub text: Option<AttrValue>,
}

/// Link component used for adding links to the league skinset finder.
#[function_component(Link)]
pub fn link(props: &LinkProps) -> Html {
    // Resolve the link text.
    let link_text = props.text.as_ref().unwrap_or(&props.href);

    if props.open_in_new_tab {
        html! {
            <a href={&props.href} target="_blank" rel={"noreferrer noopener"}>
                {link_text}
                {" "}
                <Icon icon_id={IconId::LucideExternalLink} />
            </a>
        }
    } else {
        html! {
            <a href={&props.href}> {link_text} </a>
        }
    }
}

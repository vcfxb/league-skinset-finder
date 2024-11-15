//! Link component for the league skinset finder. 
// use leptos::{component, view, IntoView};
// use leptos_icons::Icon;
// use icondata::TbExternalLink;

use yew::{function_component, html, AttrValue, Html, Properties};
use yew_icons::{Icon, IconId};

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or(false)]
    pub open_in_new_tab: bool,
    pub href: AttrValue,
    #[prop_or_default]
    pub text: Option<AttrValue>
}

/// Component that renders a link. 
/// 
/// # Arguments 
/// - `open_in_new_tab` - Should this link open in a new tab? (default: false)
/// - `href` - The URL to bring the user to when they click this link. 
/// - `text` - The text shown to the user. When [`None`], use the content of `href`. 
#[function_component]
pub fn Link(props: &Props) -> Html {
    // Resolve the text to display to the user. 
    let link_text = props.text.as_ref().unwrap_or(&props.href);

    if props.open_in_new_tab {
        html! {
            <a href={&props.href} target="_blank" rel="noreferrer noopener"> 
                {link_text} {" "} <Icon icon_id={IconId::OcticonsLinkExternal16} />
            </a>
        }
    } else {
        html! { <a href={&props.href}> {link_text} </a> }
    }
}

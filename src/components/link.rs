//! Link component for the league skinset finder. 
use leptos::{component, view, IntoView};
use leptos_icons::Icon;
use icondata::TbExternalLink;

/// Component that renders a link. 
/// 
/// # Arguments 
/// - `open_in_new_tab` - Should this link open in a new tab? (default: false)
/// - `href` - The URL to bring the user to when they click this link. 
/// - `text` - The text shown to the user. When [`None`], use the content of `href`. 
#[component]
pub fn Link(
    #[prop(optional)]
    open_in_new_tab: bool,
    #[prop(into)]
    href: String,
    #[prop(optional)]
    text: Option<String>
) -> impl IntoView {
    // Resolve the text to display to the user. 
    let link_text: &String = text.as_ref().unwrap_or(&href);

    if open_in_new_tab {
        view! {
            <a href={&href} target="_blank" rel="noreferrer noopener"> 
                {link_text} " " <Icon icon=TbExternalLink />
            </a>
        }
    } else {
        view! { <a href={&href}> {link_text} </a> }
    }
}

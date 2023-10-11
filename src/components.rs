//! Yew components to build out the League Skinset Finder frontend. 

use yew::prelude::*;
use crate::BUILT_AT;

use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
struct LinkProps {
    /// Open this link in a new tab?
    #[prop_or_default]
    open_in_new_tab: bool,
    /// The address being linked to.
    href: AttrValue,
    /// The text of the link (default to the address if none).
    #[prop_or_default]
    text: Option<AttrValue>,
}

#[function_component(Link)]
fn link(props: &LinkProps) -> Html {
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


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="mt-3 card bg-light text-dark"> 
            <div class="container p-4"> 
                <p class="h1"> {"League of Legends skinset finder"} </p>
                <p> {"This tool is used to find League of Legend team comps that share skins from the same skinset."} </p>
                <p> 
                    {"I currently source my skin data from"} 
                    <Link href="https://leagueoflegends.fandom.com/wiki/Champion_skin/Skin_themes" open_in_new_tab={true} />
                    {", and my lane data from "}
                    <Link href="https://leagueoflegends.fandom.com/wiki/List_of_champions_by_draft_position" open_in_new_tab={true} />
                    {"."}
                </p>
                <p> {"Data was last updated from these sources on "} {BUILT_AT} {"."} </p>
                <p>
                    {"
                    I will try to keep this generally up to date with league skins and champions, but may not always
                    remember to update this every patch. If you notice that the date above is a long time ago, or there
                    are champs/skins missing, please let me know by filing an Issue report at 
                    "}
                    <Link href="https://github.com/Alfriadox/league-skinset-finder/issues" open_in_new_tab={true} /> {"."}
                </p>
            </div>
        </div>
    }
}

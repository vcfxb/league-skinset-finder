//! Build script to read the skinset table and champ lanes table at compile time and generate rust code that can be
//! used in the web app.  

use scraper::{Html, Selector};
use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

/// Include the wiki sets table from https://leagueoflegends.fandom.com/wiki/Champion_skin/Skin_themes.
#[allow(unused)]
const WIKI_SETS_TABLE: &'static str = include_str!("assets/wiki-sets-table.html");

/// Include the official sets table from https://leagueoflegends.fandom.com/wiki/Champion_skin/Skin_themes.
#[allow(unused)]
const OFFICIAL_SETS_TABLE: &'static str = include_str!("assets/official-sets-table.html");

/// Select which table to use.
const SKINSETS_HTML: &'static str = WIKI_SETS_TABLE;

/// Lane table from https://leagueoflegends.fandom.com/wiki/List_of_champions_by_draft_position.
const LANES_HTML: &'static str = include_str!("assets/champ-lanes-table.html");

/// This build script generates a file that stores build data for reference in the web interface.
fn main() -> anyhow::Result<()> {
    // Re-run if this script changes.
    println!("cargo:rerun-if-changed=build.rs");

    // We manually track the data acquired time for now.
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR is defined");
    // Put all generated code in `generated.rs`.
    let dest_path = Path::new(&out_dir).join("generated.rs");
    // Create the output file.
    let out_file = File::create(&dest_path)?;
    // Create a buffered writer around the output file.
    let mut writer = BufWriter::new(out_file);
    // Write the necessary includes and structs to the output file.
    writeln!(
        &mut writer,
        "{}",
        unindent::unindent(
            r#"
        use serde::{Serialize, Deserialize};
        use enumflags2::{BitFlags, bitflags, make_bitflags};
        use derive_more::Display;

        /// Bitflaggable Lane enumeration for the available lanes in league. 
        #[bitflags]
        #[repr(u8)]
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Display, Serialize, Deserialize)]
        pub enum Lane {
            Top,
            Jungle,
            Mid,
            Bot,
            Support
        }

    "#
        )
    )?;

    writeln!(
        &mut writer,
        "{}",
        unindent::unindent(
            r#"
        /// Lane data parsed at compile time from the downloaded lane table HTML. 
        pub const LANE_DATA: &'static [(&'static str, BitFlags<Lane>)] = &[
    "#
        )
    )?;

    // Parse the lane data and add to the generated file.
    let lane_data = parse_lanes_file()
        // Iterate over the parsed output.
        .into_iter()
        // Convert to a const-evaluable string of rust code.
        .map(|(champ_name, lanes)| {
            // Connect all the champion's lanes with an 'or' operator.
            let lane_string = lanes.join(" | ");
            // Format into a string that is evaluable as a constant.
            format!("(r#\"{champ_name}\"#, make_bitflags!(Lane::{{ {lane_string} }}))")
        })
        // Join into one big long string.
        .collect::<Vec<String>>()
        // Join with commas and indents.
        .join(",\n\t");

    // Write lane data
    writeln!(&mut writer, "\t{lane_data}\n];")?;
    // Parse skinset data.
    let (champs_to_skinsets, all_skinsets) = parse_skinsets_file();

    // Add all skinset data to file.
    writeln!(
        &mut writer,
        "{}",
        unindent::unindent(
            r#"
        /// List of all skinsets parsed from skinset HTML table at compile time. 
        pub const ALL_SKINSET_NAMES: &'static [&'static str] = &[
    "#
        )
    )?;

    let all_skinset_data = all_skinsets
        .into_iter()
        // Map skinsets into raw string literals.
        .map(|skinset| format!("r##\"{skinset}\"##"))
        // Collect to vec and join into string of lines.
        .collect::<Vec<String>>()
        // Join by comma and indent.
        .join(",\n\t");

    // Close the skinset list.
    writeln!(&mut writer, "\t{all_skinset_data}\n];")?;

    // Finally write the skinset map.
    writeln!(
        &mut writer,
        "{}",
        unindent::unindent(
            r#"
        /// Map of all champ names to skinsets. 
        pub const CHAMPS_TO_SKINSETS: &'static [(&'static str, &'static [&'static str])] = &[
    "#
        )
    )?;

    // Format out the body of the skinset map data.
    let skinset_map_data = champs_to_skinsets
        .into_iter()
        .map(|(champ_name, skinsets)| {
            // Make the body of the skinset list string.
            let skinsets_data = skinsets
                .into_iter()
                .map(|skinset_name| format!("r##\"{skinset_name}\"##"))
                .collect::<Vec<String>>()
                .join(", ");

            format!("(r##\"{champ_name}\"##, &[{skinsets_data}])")
        })
        .collect::<Vec<String>>()
        .join(",\n\t");

    writeln!(&mut writer, "\t{skinset_map_data}\n];")?;

    // Flush any unwritten content.
    writer.flush()?;

    // Build status exits OK.
    Ok(())
}

/// Parse the skinsets file from html and return a map from champ name -> skinsets
/// and a set of all the skinset names.
///
/// Adapted from original runtime version.
fn parse_skinsets_file() -> (HashMap<String, HashSet<String>>, HashSet<String>) {
    // Parse the fragment we're useing.
    let fragment = Html::parse_fragment(SKINSETS_HTML);
    // Make a selector to get rows out of the table.
    let rows_selector: Selector = Selector::parse("tr").expect("rows selector good");
    // Make a selector to find champs from a row element.
    let champs_selector: Selector = Selector::parse("li > span").expect("champ selector good");
    // Make a selector to find the set name from a row ref.
    let set_name_selector: Selector =
        Selector::parse("th:last-of-type").expect("set name selector good");
    // Make an iterator to go over all the rows of the skinset table, skipping the header row.
    let row_iter = fragment.select(&rows_selector).skip(1);
    // Make the champ-skinset map to populate
    let mut champ_to_skinset_map: HashMap<String, HashSet<String>> = HashMap::new();
    // Make set of all skinsets to store and pass out too.
    let mut set_of_all_skinsets: HashSet<String> = HashSet::new();

    // Iterate over all the rows of the table.
    for row_ref in row_iter {
        // Get the set name.
        let set_name: String = row_ref
            .select(&set_name_selector)
            .next()
            .expect("finds set name")
            .text()
            .collect::<String>();

        // Insert/upsert into the set of all skinsets.
        set_of_all_skinsets.insert(set_name.clone());

        // Get an iterator over all the champ names in this set.
        let champs_iter = row_ref.select(&champs_selector).map(|champ_el_ref| {
            champ_el_ref
                // Get the referenced element
                .value()
                // Read the 'data-champion' attribute
                .attr("data-champion")
                // Crash if it's not there.
                .expect("champion name available")
                // Convert to owned string
                .to_owned()
        });

        // Add all of the champ-skinset mappings into the map.
        for champ in champs_iter {
            champ_to_skinset_map
                // Get the map entry for this champ.
                .entry(champ)
                // Make a new empty one if not recognized.
                .or_default()
                // Add the set name to this champ's list.
                .insert(set_name.clone());
        }
    }

    (champ_to_skinset_map, set_of_all_skinsets)
}

/// Parse the lanes table file from html and return a map from champ name -> lanes.
///
/// Adapted from original runtime version.
fn parse_lanes_file() -> HashMap<String, Vec<&'static str>> {
    // Make map to populate.
    let mut champ_to_lanes_map = HashMap::new();
    // Load the table fragment into a scrapable document.
    let fragment: Html = Html::parse_fragment(LANES_HTML);
    // Make a selector to get rows out of the table.
    let rows_selector: Selector = Selector::parse("tbody > tr").expect("rows selector good");
    // Make a selector to get columns out of a row element.
    let cols_selector: Selector = Selector::parse("td").expect("cols selector good");

    // Iterate over every row in the table.
    for row_ref in fragment.select(&rows_selector) {
        // Assume the children of the row are (in order):
        // Champ name, top, jungle, mid, bot lane, support, unused.

        // Make an iterator over column elements that takes the exact number we want.
        let mut cols_iterator = row_ref
            .select(&cols_selector)
            // Ignore the unused column.
            .take(6);

        // Take the champ name from the iterator.
        let champ_name: String = cols_iterator
            .next()
            .expect("Finds champ name element")
            .value()
            .attr("data-sort-value")
            .expect("Finds champ name attribute")
            .to_owned();

        // Make a list of lanes to populate by iterating over the rest of the columns.
        let mut lanes: Vec<&'static str> = Vec::with_capacity(5);
        // Iterate over the remaining columns to populate bitflags.
        for (index, col) in cols_iterator.enumerate() {
            if col.value().attr("data-sort-value").is_some() {
                match index {
                    0 => lanes.push("Top"),
                    1 => lanes.push("Jungle"),
                    2 => lanes.push("Mid"),
                    3 => lanes.push("Bot"),
                    4 => lanes.push("Support"),
                    // Unreachable because we limit the number of <td> tags in the iterator using take()
                    _ => unreachable!(),
                }
            }
        }

        // Add the champ and their lanes to the map. We use insert rather than upsert here because we assume there
        // are no duplicates in the table.
        champ_to_lanes_map.insert(champ_name, lanes);
    }

    champ_to_lanes_map
}


use instant::Instant;
use log::{Level, info};
use crate::components::App;
use crate::lanes::GLOBAL_LANES_MAP;
use crate::skinsets::GLOBAL_SKINSETS_MAP;

mod lanes;
mod skinsets;
mod players;
mod components;

fn main() {
    // Track the starting instant for performance reasons. 
    let start: Instant = Instant::now();
    // Initialize the logger.
    console_log::init_with_level(Level::Info).expect("error initializing logger");
    info!("Logger started");
    // Force init of lazy thread local statics -- if the logger is set to `Debug` then this will almost certainly 
    // take forever -- the scraper generates tons of debug log messages. 
    GLOBAL_LANES_MAP.with(|_| {});
    GLOBAL_SKINSETS_MAP.with(|_| {});
    // Save the time it took to intialize the maps and logger.
    let init_time: Instant = Instant::now();
    // Log how much time it took to scrape/initialize these globals.
    info!("Globals initialized in {:?}", init_time - start);
    // Start the yew global renderer -- this kicks off a loading bar to wait for everything to initialize. 
    yew::Renderer::<App>::new().render();
}

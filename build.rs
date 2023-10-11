use std::{env, path::Path, fs, time::Instant};

/// This build script generates a file that stores build data for reference in the web interface. 
fn main() -> anyhow::Result<()> {
    // Ensure that this build script gets re-run every single time. 
    env::set_var("REBUILD", format!("{:?}", Instant::now()));
    println!("cargo:rerun-if-env-changed=REBUILD");
    
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR is defined");
    // Put all generated code in `generated.rs`.
    let dest_path = Path::new(&out_dir).join("generated.rs");
    // Get the current timestamp formatted as somewhat human readable. 
    let timestamp = chrono::Local::now().to_rfc2822();
    // Write the timestamp to the output file.
    fs::write(&dest_path, format!("pub const BUILT_AT: &'static str = \"{timestamp}\";"))?;
    
    // Build status exits OK.
    Ok(())
}

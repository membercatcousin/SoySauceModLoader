use std::error::Error;
use ssml::builder::build_modded_jar;
use ssml::config::Config;
use ssml::name::{name, name_short};
use ssml::scanner::scan_mods_dir;

fn main() -> Result<(), Box<dyn Error>> {
    println!("{name} ({name_short}) v0.1.0");
    println!("=================================");
    println!();

    let config = Config::default();

    // 1. Check if unmodded.jar exists
    if !config.ensure_unmodded_jar_exists() {
        return Ok(());
    }

    // 2. Ensure mods/ folder exists
    config.ensure_mods_dir_exists()?;

    // 3. Scan mods/ and collect all JSON recipes and splash texts
    let mod_files = scan_mods_dir(&config.mods_dir)?;

    // 4. Open clean base jar and build target jar
    let stats = build_modded_jar(&config.unmodded_path, &config.modded_path, &mod_files)?;

    println!("[SUCCESS] Build complete.");
    println!("  - Stripped {} META-INF entries", stats.stripped_meta);
    println!("  - Total modded entries applied: {}", stats.modded_entries);
    println!(
        "Built '{}'. It has the modded version of the game. Use it to launch minecraft.",
        config.modded_path.display()
    );

    Ok(())
}

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

use crate::patcher;

/// Statistics gathered during the jar building process.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BuildStats {
    pub stripped_meta: usize,
    pub overwritten: usize,
    pub modded_entries: usize,
}

/// Builds the final modded jar by combining the unmodded base jar with collected mod files.
pub fn build_modded_jar(
    unmodded_path: &Path,
    modded_path: &Path,
    mod_files: &HashMap<String, Vec<u8>>,
) -> Result<BuildStats, Box<dyn std::error::Error>> {
    println!("\n[INFO] Building '{}'...", modded_path.display());

    let base_file = File::open(unmodded_path)?;
    let mut base_archive = ZipArchive::new(base_file)?;

    let target_file = File::create(modded_path)?;
    let mut writer = ZipWriter::new(target_file);

    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated);

    let mut stripped_meta = 0;
    let mut overwritten = 0;

    // 1. Copy all files from unmodded.jar EXCEPT META-INF and overwritten entries
    for i in 0..base_archive.len() {
        let mut file = base_archive.by_index(i)?;
        let name = file.name().to_string();

        // Strip META-INF
        if name.starts_with("META-INF/") || name == "META-INF" {
            stripped_meta += 1;
            continue;
        }

        // Overwrite if mod provided a replacement
        if mod_files.contains_key(&name) {
            overwritten += 1;
            continue;
        }

        // Apply class bytecode patches if applicable
        if patcher::should_patch_class(&name) {
            let mut class_bytes = Vec::new();
            file.read_to_end(&mut class_bytes)?;

            let (patched_bytes, _was_patched) = patcher::patch_class(&name, &class_bytes);

            writer.start_file(&name, options)?;
            writer.write_all(&patched_bytes)?;
            continue;
        }

        writer.start_file(&name, options)?;
        std::io::copy(&mut file, &mut writer)?;
    }

    // 2. Write all modded recipes, splashes, and assets into the jar
    for (target_path, content) in mod_files {
        writer.start_file(target_path, options)?;
        writer.write_all(content)?;
    }

    writer.finish()?;

    Ok(BuildStats {
        stripped_meta,
        overwritten,
        modded_entries: mod_files.len(),
    })
}

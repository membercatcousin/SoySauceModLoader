use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use walkdir::WalkDir;
use zip::ZipArchive;

/// Resolves the destination path inside the modded jar for a given mod file entry.
///
/// Returns:
/// - `Some(target_path)` if the file is a valid mod resource (.json or splashes.txt).
/// - `None` if the file should be ignored.
pub fn resolve_mod_target_path(name: &str) -> Option<String> {
    let is_json = name.ends_with(".json");
    let is_splash = name.ends_with("splashes.txt");

    if !is_json && !is_splash {
        return None;
    }

    let target_path = if name.starts_with("data/") || name.starts_with("assets/") {
        name.to_string()
    } else if is_splash {
        "assets/minecraft/texts/splashes.txt".to_string()
    } else {
        let file_name = Path::new(name)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(name);
        format!("data/minecraft/recipes/{}", file_name)
    };

    Some(target_path)
}

/// Reads a single zip mod archive and inserts its resource entries into `mod_files`.
pub fn load_mod_archive<P: AsRef<Path>>(
    path: P,
    mod_files: &mut HashMap<String, Vec<u8>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = path.as_ref();
    println!("[INFO] Loading Mod: {}", path.display());

    let file = File::open(path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut zip_file = archive.by_index(i)?;
        let name = zip_file.name().to_string();

        if zip_file.is_dir() {
            continue;
        }

        match resolve_mod_target_path(&name) {
            Some(target_path) => {
                let mut content = Vec::new();
                zip_file.read_to_end(&mut content)?;
                println!("[INFO] Injecting {} into 'modded.jar'...", target_path);
                mod_files.insert(target_path, content);
            }
            None => {
                println!("[WARNING] Ignoring non-JSON/TXT file: {}", name);
            }
        }
    }

    Ok(())
}

/// Scans the given directory recursively for `.zip` mod archives and collects their contents.
pub fn scan_mods_dir<P: AsRef<Path>>(
    mods_dir: P,
) -> Result<HashMap<String, Vec<u8>>, Box<dyn std::error::Error>> {
    let mut mod_files = HashMap::new();

    for entry in WalkDir::new(mods_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("zip") {
            load_mod_archive(path, &mut mod_files)?;
        }
    }

    Ok(mod_files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_mod_target_path_preserved_paths() {
        assert_eq!(
            resolve_mod_target_path("data/minecraft/recipes/stone.json"),
            Some("data/minecraft/recipes/stone.json".to_string())
        );
        assert_eq!(
            resolve_mod_target_path("assets/minecraft/texts/splashes.txt"),
            Some("assets/minecraft/texts/splashes.txt".to_string())
        );
    }

    #[test]
    fn test_resolve_mod_target_path_root_splash() {
        assert_eq!(
            resolve_mod_target_path("splashes.txt"),
            Some("assets/minecraft/texts/splashes.txt".to_string())
        );
        assert_eq!(
            resolve_mod_target_path("foo/splashes.txt"),
            Some("assets/minecraft/texts/splashes.txt".to_string())
        );
    }

    #[test]
    fn test_resolve_mod_target_path_bare_recipe() {
        assert_eq!(
            resolve_mod_target_path("custom_sword.json"),
            Some("data/minecraft/recipes/custom_sword.json".to_string())
        );
        assert_eq!(
            resolve_mod_target_path("nested/folder/custom_sword.json"),
            Some("data/minecraft/recipes/custom_sword.json".to_string())
        );
    }

    #[test]
    fn test_resolve_mod_target_path_ignored_files() {
        assert_eq!(resolve_mod_target_path("README.md"), None);
        assert_eq!(resolve_mod_target_path("pack.png"), None);
        assert_eq!(resolve_mod_target_path("mod.info"), None);
    }
}

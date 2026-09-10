use std::fs;
use std::path::PathBuf;

pub const DEFAULT_UNMODDED_JAR: &str = "unmodded.jar";
pub const DEFAULT_MODDED_JAR: &str = "modded.jar";
pub const DEFAULT_MODS_DIR: &str = "mods";

pub struct Config {
    pub unmodded_path: PathBuf,
    pub modded_path: PathBuf,
    pub mods_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            unmodded_path: PathBuf::from(DEFAULT_UNMODDED_JAR),
            modded_path: PathBuf::from(DEFAULT_MODDED_JAR),
            mods_dir: PathBuf::from(DEFAULT_MODS_DIR),
        }
    }
}

impl Config {
    /// Checks if the unmodded base jar exists.
    /// Prints error and instructions if not found.
    pub fn ensure_unmodded_jar_exists(&self) -> bool {
        if !self.unmodded_path.exists() {
            eprintln!("[ERROR] '{}' not found.", self.unmodded_path.display());
            eprintln!(
                "[INFO] Please place your vanilla 1.13.2 jar here and name it '{}'.",
                self.unmodded_path.display()
            );
            return false;
        }
        true
    }

    /// Ensures that the mods directory exists, creating it if necessary.
    pub fn ensure_mods_dir_exists(&self) -> std::io::Result<()> {
        if !self.mods_dir.exists() {
            fs::create_dir_all(&self.mods_dir)?;
            println!("[INFO] Created '{}' directory.", self.mods_dir.display());
        }
        Ok(())
    }
}

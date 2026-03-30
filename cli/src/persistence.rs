use ron::de::from_str;
use ron::ser::{PrettyConfig, to_string_pretty};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;

/// Returns `~/.local/share/dicebag/`, creating it if needed.
pub fn data_dir() -> PathBuf {
    let dir = dirs::data_dir()
        .expect("Could not determine user data directory")
        .join("dicebag");
    fs::create_dir_all(&dir).expect("Could not create dicebag data directory");
    dir
}

/// Returns the subdirectory for a campaign: `<data_dir>/<id>-<name-with-hyphens>/`.
/// Creates the directory if it doesn't exist.
pub fn campaign_dir(campaign_id: i32, campaign_name: &str) -> PathBuf {
    let slug = campaign_name.to_lowercase().replace(' ', "-");
    let dir = data_dir().join(format!("{}-{}", campaign_id, slug));
    fs::create_dir_all(&dir).expect("Could not create campaign directory");
    dir
}

/// Finds an existing campaign directory by name (case-insensitive slug match).
/// Returns None if no matching directory exists.
pub fn find_campaign_dir(campaign_name: &str) -> Option<PathBuf> {
    let slug = campaign_name.to_lowercase().replace(' ', "-");
    let base = data_dir();
    let entries = fs::read_dir(&base).ok()?;
    for entry in entries.flatten() {
        let fname = entry.file_name();
        let fname = fname.to_string_lossy();
        // Directory name format: <id>-<slug>
        if let Some(pos) = fname.find('-')
            && fname[pos + 1..] == *slug
        {
            let path = entry.path();
            if path.is_dir() {
                return Some(path);
            }
        }
    }
    None
}

/// Lists all campaign directories in the data dir.
pub fn list_campaign_dirs() -> Vec<PathBuf> {
    let base = data_dir();
    let Ok(entries) = fs::read_dir(&base) else {
        return vec![];
    };
    let mut dirs: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    dirs.sort();
    dirs
}

pub fn save<T: Serialize>(value: &T, path: &PathBuf) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = File::create(path)?;
    let ron = to_string_pretty(value, PrettyConfig::default()).expect("Serialization failed");
    file.write_all(ron.as_bytes())?;
    Ok(())
}

pub fn load<T: for<'de> Deserialize<'de>>(path: &PathBuf) -> io::Result<T> {
    let mut file = File::open(path)?;
    let mut ron = String::new();
    file.read_to_string(&mut ron)?;
    Ok(from_str(&ron).expect("Deserialization failed"))
}

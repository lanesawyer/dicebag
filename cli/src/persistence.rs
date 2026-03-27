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

pub fn save<T: Serialize>(value: &T, filename: &str) -> io::Result<()> {
    let path = data_dir().join(filename.replace(" ", "-"));
    let mut file = File::create(path)?;
    let ron = to_string_pretty(value, PrettyConfig::default()).expect("Serialization failed");
    file.write_all(ron.as_bytes())?;
    Ok(())
}

pub fn load<T: for<'de> Deserialize<'de>>(filename: &str) -> io::Result<T> {
    let path = data_dir().join(filename.replace(" ", "-"));
    let mut file = File::open(path)?;
    let mut ron = String::new();
    file.read_to_string(&mut ron)?;
    Ok(from_str(&ron).expect("Deserialization failed"))
}

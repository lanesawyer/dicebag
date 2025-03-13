use ron::de::from_str;
use ron::ser::to_string;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read, Write};

// TODO: More than just a simple RON file, probably should support some real DB someday

pub trait Persistable: Serialize + for<'de> Deserialize<'de> {
    fn save_to_ron_file(&self, filename: &str) -> io::Result<()> {
        let mut file = File::create(filename.replace(" ", "-"))?;
        let ron = to_string(self).expect("Serialization failed");
        file.write_all(ron.as_bytes())?;
        Ok(())
    }

    fn load_from_ron_file(filename: &str) -> io::Result<Self> {
        let mut file = File::open(filename.replace(" ", "-"))?;
        let mut ron = String::new();
        file.read_to_string(&mut ron)?;
        let obj: Self = from_str(&ron).expect("Deserialization failed");
        Ok(obj)
    }
}

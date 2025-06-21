use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};
use serde::{Serialize, de::DeserializeOwned};
use std::path::Path;

pub fn load_json<T: DeserializeOwned>(path: &str) -> Result<T, String> {
    if !Path::new(path).exists() {
        return Err(format!("File {} not found", path));
    }
    let file = File::open(path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).map_err(|e| e.to_string())
}

pub fn save_json<T: Serialize>(path: &str, data: &T) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .map_err(|e| e.to_string())?;

    let content = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
    file.write_all(content.as_bytes()).map_err(|e| e.to_string())
}

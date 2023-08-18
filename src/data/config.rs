use super::get_root_dir;
use serde::de::DeserializeOwned;
use serde_json::from_reader;
use std::io::{Error as IoError, Read};
use std::path::PathBuf;

pub fn get_config<'a, C, R>(reader: R) -> Result<C, IoError>
where
    C: DeserializeOwned,
    R: Read,
{
    Ok(from_reader(reader)?)
}

pub fn get_config_dir(name: &str) -> PathBuf {
    get_root_dir(name, "config".to_string())
}

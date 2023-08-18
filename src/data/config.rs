use super::get_root_dir;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_reader, to_writer, Error as SerdeError};
use std::io::prelude::{Read, Write};
use std::path::PathBuf;

pub fn get_config<C, R>(reader: R) -> Result<C, SerdeError>
where
    C: DeserializeOwned,
    R: Read,
{
    from_reader(reader)
}

pub fn write_config<C, W>(writer: &mut W, object: &C) -> Result<(), SerdeError>
where
    W: Write,
    C: Serialize + ?Sized,
{
    to_writer(writer, object)
}

pub fn get_config_dir(name: &str) -> PathBuf {
    get_root_dir(name, Some("config".to_string()))
}

pub fn get_main_config(name: &str) -> PathBuf {
    get_root_dir(name, None).join("config.json")
}

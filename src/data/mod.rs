mod config;

pub use config::*;
use platform_dirs::AppDirs;
use std::path::{Path, PathBuf};

pub fn get_root_dir<S: AsRef<Path>>(name: &str, subdir: Option<S>) -> PathBuf {
    let dir = AppDirs::new(Some(name), false).unwrap().data_dir;

    if let Some(subdir) = subdir {
        dir.join(subdir)
    } else {
        dir
    }
    .canonicalize()
    .unwrap()
}

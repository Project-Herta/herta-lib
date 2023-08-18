use platform_dirs::AppDirs;
use std::path::PathBuf;
mod config;

pub fn get_root_dir(name: &str, subdir: String) -> PathBuf {
    AppDirs::new(Some(name), false)
        .unwrap()
        .data_dir
        .join(subdir)
}

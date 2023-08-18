use platform_dirs::AppDirs;
use std::path::PathBuf;
mod config;

pub fn get_root_dir(name: &str, subdir: Option<String>) -> PathBuf {
    let dir = AppDirs::new(Some(name), false).unwrap().data_dir;

    if let Some(subdir) = subdir {
        dir.join(subdir)
    } else {
        dir
    }
}

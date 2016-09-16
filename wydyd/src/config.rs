use std::env;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

pub fn config_dir() -> PathBuf {
    let dir_str = match env::var("XDG_CONFIG_HOME") {
        Ok(d) => format!("{}/wydy", d),
        Err(_) => {
            let home = env::var("HOME").unwrap();
            let home_path = Path::new(&home);
            let config_path = home_path.join(".config");
            if config_path.exists() {
                format!("{}/.config/wydy", home)
            } else {
                format!("{}/.wydy", home)
            }
        }
    };

    let path = Path::new(&dir_str).to_path_buf();
    if !path.exists() {
        create_dir_all(&path).unwrap();
    }
    path
}
use APP_INFO;
use app_dirs::{AppDataType, app_root};
use command::{WCommand, WLocation};
use env::Vars;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Transform a string in a script path.
///
/// # Example
///
/// update_all -> <config_dir>/scripts/update/all
pub fn scriptify(content: &str) -> Vec<PathBuf> {
    let content = content.replace("_", "/");
    let skip_first = content.starts_with("script");
    let mut scripts = content.split_whitespace();
    if skip_first {
        scripts.next();
    }
    let mut paths = Vec::new();
    for script in scripts {
        let mut script = script.to_string();
        if cfg!(target_os = "windows") {
            script.push_str(".bat");
        }
        let idx = script.rfind('/').unwrap_or(0);
        let mut path = app_root(AppDataType::UserConfig, &APP_INFO).unwrap().join("scripts");
        match idx {
            0 => {}
            _ => {
                let dirs = script.drain(..idx + 1).collect::<String>();
                path = path.join(dirs);
            }
        }
        paths.push(path.join(script));
    }
    paths
}

// TODO use WCommand list
/// Create file for script and add a WCommand to inform it.
pub fn add_script(_command_list: &mut Vec<WCommand>, path: PathBuf) {
    if path.exists() {
        warn!("Script {} exists", path.display());
        return;
    }
    match fs::create_dir_all(path.parent().unwrap()) {
        Ok(_) => {}
        Err(e) => {
            error!("{}", e);
            return;
        }
    }
    debug!("{}", path.display());
    match fs::File::create(&path) {
        Ok(_) => {}
        Err(e) => {
            error!("{}", e);
            return;
        }
    }
    #[cfg(unix)]
    {
        let mut perms = fs::metadata(&path).unwrap().permissions();
        perms.set_mode(0o744);
        fs::set_permissions(&path, perms).unwrap();
    }
}

/// Create WCommand to edit a script.
pub fn edit_script(command_list: &mut Vec<WCommand>,
                   vars: &Arc<Mutex<Vars>>,
                   path: PathBuf,
                   content: &str) {
    if !path.exists() {
        return;
    }
    let editor = vars.lock().unwrap().value_of("editor").unwrap_or("vi".to_string());
    // TODO send process to the client
    command_list.push(WCommand::new(format!("{} {}", editor, path.display()),
                                    format!("edit script {}", content),
                                    WLocation::Client));
}

// TODO same as add script
/// Delete a script and add a WCommand to inform it.
pub fn delete_script(_command_list: &mut Vec<WCommand>, path: PathBuf) {
    if !path.exists() {
        return;
    }
    debug!("Removing {}...", path.display());
    fs::remove_file(&path).unwrap();
}

/// Create WCommand to run a script.
pub fn run_script(command_list: &mut Vec<WCommand>, path: PathBuf, content: &str) {
    if !path.exists() {
        return;
    }
    command_list.push(WCommand::new(format!("{}", path.display()),
                                    format!("run script {}", content),
                                    WLocation::Both));
}

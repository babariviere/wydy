use config::config_dir;
use env::Vars;
use std::fs::{create_dir_all, File};
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex};

/// Represent a wydy command
/// command var is the command to execute, ej: vi src/command.rs
/// desc var is the description, ej: edit file "src/command.rs"
#[derive(Clone)]
pub struct WCommand {
    command: String,
    desc: String,
}

impl WCommand {
    pub fn new<S: Into<String>>(command: S, desc: S) -> WCommand {
        WCommand {
            command: command.into(),
            desc: desc.into(),
        }
    }

    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn desc(&self) -> &str {
        &self.desc
    }

    /// Run the command
    pub fn run(&self) -> i32 {
        let mut command_split = self.command.split_whitespace();
        let command_str = command_split.next().unwrap();
        let command_args = command_split.collect::<Vec<&str>>();
        let mut command = Command::new(command_str);
        command.args(command_args.as_slice());
        let output = command.output().unwrap();
        output.status.code().unwrap_or(0)
    }
}

/// Parse one command and return wydy command
/// ej:
/// command = "edit update"
/// There will be two result
/// [1] edit file update
/// [2] search for "edit update"
pub fn parse_command(command: String, vars: &Arc<Mutex<Vars>>) -> Vec<WCommand> {
    let command = command.trim().to_string();
    let mut command_clone = command.clone();
    let mut command_split = command.split_whitespace();
    let mut result = Vec::new();
    match command_split.next() {
        Some("search") => {
            let search = command_clone.drain(7..).collect::<String>();
            web_search(&search, &mut result, vars);
        }
        Some("open") => {
            let search = command_clone.drain(5..).collect::<String>();
            web_search(&search, &mut result, vars);
        }
        Some("add") => {
            if let Some("script") = command_split.next() {
                let name = command_clone.drain(11..).collect::<String>();
                add_script(&name);
            }
        }
        Some(_) => {
            if is_command(&command_clone) {
                let command = WCommand::new(command_clone.as_str(),
                                            &format!("executing {}", &command_clone));
                result.push(command);
            }
            web_search(&command_clone, &mut result, vars);
        }
        _ => {}
    }
    result
}

/// Add a script
fn add_script(name: &str) {
    let mut name = name.replace("_", "/");
    let idx = name.rfind('/').unwrap_or(0);
    let dirs = name.drain(..idx + 1).collect::<String>();
    let path = config_dir().join("scripts").join(&dirs);
    create_dir_all(&path).unwrap();
    let path = path.join(&name);
    debug!("{}", path.display());
    File::create(&path).unwrap();
}

/// Check if command is in path
fn is_command(command: &str) -> bool {
    let command = match command.split_whitespace().next() {
        Some(c) => c,
        None => command,
    };
    let path = env!("PATH");
    let path_split = path.split(':');
    for p in path_split {
        let command_path = Path::new(p).join(command);
        if command_path.exists() {
            return true;
        }
    }
    false
}

fn web_search(search: &str, commands: &mut Vec<WCommand>, vars: &Arc<Mutex<Vars>>) {
    let vars_lock = vars.lock().unwrap();
    let browser = vars_lock.value_of("browser").unwrap_or("firefox".to_string());
    let search = search.replace(" ", "%20");
    if ::url_check::is_url(&search) {
        let command = WCommand::new(format!("{} {}", browser, search),
                                    format!("opening url {}", search));
        commands.push(command);
    }
    let search_engine = vars_lock.value_of("search_engine").unwrap_or_default();
    let command = WCommand::new(format!("{} {}",
                                        browser,
                                        search_engine_link(&search_engine, &search)),
                                format!("search for {}", search));
    commands.push(command);
}

/// With the name of the search engine and the search to do, it returns a link to the search on the
/// search engine.
fn search_engine_link(name: &str, search: &str) -> String {
    match name {
        "duckduckgo" => format!("https://duckduckgo.com/?q={}", search),
        "google" => format!("https://google.com/#q={}", search),
        s => {
            error!("Unknown search engine {}, searching on duckduckgo by default.\nType \"edit \
                    vars\" and change the value of \"search_engine\" to fix this problem.",
                   s);
            format!("https://duckduckgo.com/?q={}", search)
        }
    }
}

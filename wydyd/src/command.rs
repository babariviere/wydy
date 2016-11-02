use APP_INFO;
use app_dirs::{AppDataType, get_app_root};
use env::Vars;
use parser::{WCPResult, WKeyword, parse_command_str};
use std::fs::{create_dir_all, File};
use std::path::{Path, PathBuf};
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

// TODO add wlocation to wcommand
// Specify where the command can be run
#[allow(dead_code)]
enum WLocation {
    Both,
    Server,
    Client,
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

/// Parse user command and return a list of wydy commands.
///
/// # Example
///
/// user_command = "edit update":
/// [1] edit file update
/// [2] search for "edit update"
pub fn parse_user_command(command: String, vars: &Arc<Mutex<Vars>>) -> Vec<WCommand> {
    let mut command_list = Vec::new();
    let parse_result = parse_command_str(command);
    debug!("Parse result {:?}", parse_result);

    script_cmd(&mut command_list, &parse_result, vars);
    command_cmd(&mut command_list, &parse_result);
    web_search_cmd(&mut command_list, &parse_result, vars);

    command_list
}

/// Add all command related to script
fn script_cmd(command_list: &mut Vec<WCommand>,
              parse_result: &WCPResult,
              vars: &Arc<Mutex<Vars>>) {
    let &(ref keyword, ref content) = parse_result;
    // TODO add support for multiple script
    if !content.starts_with("script") {
        return;
    }
    let mut content = content.clone();
    let content = content.drain(7..).collect::<String>();
    // TODO add support to check script presence and run it
    match *keyword {
        WKeyword::Add => {
            let path = scriptify(&content);
            create_dir_all(path.parent().unwrap()).unwrap();
            debug!("{}", path.display());
            File::create(&path).unwrap();
        }
        WKeyword::Edit => {
            let path = scriptify(&content);
            if !path.exists() {
                return;
            }
            let editor = vars.lock().unwrap().value_of("editor").unwrap_or("vi".to_string());
            // TODO send process to the client
            command_list.push(WCommand::new(format!("{} {}", editor, path.display()),
                                            format!("edit script {}", content)));
        }
        _ => {}
    }
}

/// Transform a string in a script path.
///
/// # Example
///
/// update_all -> <config_dir>/scripts/update/all
fn scriptify(name: &str) -> PathBuf {
    let mut name = name.replace("_", "/");
    let idx = name.rfind('/').unwrap_or(0);
    let mut path = get_app_root(AppDataType::UserConfig, &APP_INFO).unwrap().join("scripts");
    match idx {
        0 => {}
        _ => {
            let dirs = name.drain(..idx + 1).collect::<String>();
            path = path.join(dirs);
        }
    }
    path = path.join(name);
    path
}

/// Check if command is in path and add it to the command list.
fn command_cmd(command_list: &mut Vec<WCommand>, parse_result: &WCPResult) {
    match *parse_result {
        (WKeyword::Run, ref s) |
        (WKeyword::None, ref s) => {
            let command = match s.split_whitespace().next() {
                Some(c) => c,
                None => s,
            };
            let path = env!("PATH");
            let path_split = path.split(':');
            let mut exists = false;
            for p in path_split {
                let command_path = Path::new(p).join(command);
                if command_path.exists() {
                    exists = true;
                }
            }
            if exists {
                command_list.push(WCommand::new(s.to_string(), format!("execute `{}`", s)));
            }
        }
        _ => {}
    }
}

/// Check the parse result and add to the command list a link or a search.
fn web_search_cmd(command_list: &mut Vec<WCommand>,
                  parse_result: &WCPResult,
                  vars: &Arc<Mutex<Vars>>) {
    let vars_lock = vars.lock().unwrap();
    let browser = vars_lock.value_of("browser").unwrap_or("firefox".to_string());
    let search_engine = vars_lock.value_of("search_engine").unwrap_or_default();
    let &(ref keyword, ref search) = parse_result;
    let search = search.replace(" ", "%20");
    match *keyword {
        WKeyword::Search | WKeyword::None => {
            if ::url_check::is_url(&search) {
                command_list.push(WCommand::new(format!("{} {}", browser, search),
                                                format!("opening url {}", search)));
            }
            command_list.push(WCommand::new(format!("{} {}",
                                                    browser,
                                                    search_engine_link(&search_engine, &search)),
                                            format!("search for {}", search)));
        }
        WKeyword::Open => {
            if ::url_check::is_url(&search) {
                command_list.push(WCommand::new(format!("{} {}", browser, search),
                                                format!("opening url {}", search)));
            }
        }
        _ => {}
    }
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

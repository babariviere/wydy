use std::path::Path;
use std::process::Command;

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
/// command = "edit update_all"
/// There will be two result
/// [1] edit file update_all
/// [2] search for "edit update_all"
pub fn parse_command(command: String) -> Vec<WCommand> {
    let command = command.trim().to_string();
    let mut command_clone = command.clone();
    let mut command_split = command.split_whitespace();
    let mut result = Vec::new();
    match command_split.next() {
        Some("search ") => {
            let search = command_clone.drain(7..).collect::<String>();
            let command = web_search(&search);
            result.push(command);
        }
        #[cfg(feature="all")]
        Some("open ") => {}
        Some(_) => {
            // TODO check if it's a command
            // If it's a command, choose from multiple choice
            if is_command(&command_clone) {
                let command = WCommand::new(command_clone.as_str(),
                                            &format!("executing {}", &command_clone));
                result.push(command);
            }
            let command_web = web_search(&command_clone);
            result.push(command_web);
        }
        _ => {}
    }
    result
}

/// Check if command is in path
fn is_command(command: &str) -> bool {
    let command = match command.split_whitespace().next() {
        Some(c) => c,
        None => command,
    };
    let path = env!("PATH");
    let path_split = path.split(":");
    for p in path_split {
        let command_path = Path::new(p).join(command);
        if command_path.exists() {
            return true;
        }
    }
    false
}

#[cfg(not(any(feature="url-check", feature="all")))]
fn web_search(search: &str) -> WCommand {
    let search = search.replace(" ", "%20");
    WCommand::new(format!("firefox https://duckduckgo.com/?q={}", search),
                  format!("search for {}", search))
}

#[cfg(any(feature="url-check", feature="all"))]
fn web_search(search: &str) -> WCommand {
    // TODO add variable for search engine
    let search = search.replace(" ", "%20");
    let command = match ::url_check::is_url(&search) {
        true => {
            WCommand::new(format!("firefox {}", search),
                          format!("opening url {}", search))
        }
        false => {
            WCommand::new(format!("firefox https://duckduckgo.com/?q={}", search),
                          format!("search for {}", search))
        }
    };
    command
}

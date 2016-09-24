use std::process::Command;

/// Represent a wydy command 
/// command var is the command to execute, ej: vi src/command.rs
/// desc var is the description, ej: edit file "src/command.rs"
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
    pub fn run(&self) {
        let mut command_split = self.command.split_whitespace();
        let command = command_split.next().unwrap();
        let command_args = command_split.collect::<Vec<&str>>();
        Command::new(command).args(command_args.as_slice()).spawn().unwrap();
    }
}

/// Parse one command and return wydy command 
/// ej: 
/// command = "edit update_all"
/// There will be two result
/// [1] edit file update_all
/// [2] search for "edit update_all"
pub fn parse_command(command: String) -> Vec<WCommand> {
    let mut command_split = command.split_whitespace();
    let mut result = Vec::new();
    match command_split.next() {
        Some("search") => {
            // TODO fix attached string => no space
            let search = string_with_space(command_split);
            let command = web_search(search);
            result.push(command);
        }
        Some(s) => {
            let search = format!("{} {}", s, string_with_space(command_split));
            let command = web_search(search);
            result.push(command);
        }
        _ => {}
    }
    result
}

fn string_with_space(splitted: ::std::str::SplitWhitespace) -> String {
    splitted.map(|x| format!("{} ", x)).collect()
}

fn web_search(search: String) -> WCommand {
            let command = WCommand::new(format!("firefox https://duckduckgo.com/?q={}", search), format!("search for {}", search));
            command
}

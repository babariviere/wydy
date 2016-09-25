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
    let mut command_split = command.split_whitespace();
    let mut result = Vec::new();
    match command_split.next() {
        Some("search") => {
            // TODO fix attached string => no space
            let search = string_with_space(command_split);
            let command = web_search(search);
            result.push(command);
        }
        Some("open") if cfg!(feature = "url-check") == true => {
            println!("OPEN");
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

#[cfg(not(feature="url-check"))]
fn web_search(search: String) -> WCommand {
    WCommand::new(format!("firefox https://duckduckgo.com/?q={}", search),
                  format!("search for {}", search))
}

#[cfg(feature="url-check")]
fn web_search(search: String) -> WCommand {
    // TODO add variable for search engine
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

use config::*;
use std::process::Command;

struct Variable {
    name: String,
    value: String,
}

/// Start the editor to edit wydy variables.
pub fn edit_variables(editor: String) {
    let config_dir = config_dir();
    let env_file = config_dir.join("vars");
    Command::new(editor).arg(env_file.display().to_string()).output();
}

use config::config_dir;
use std::process::Command;

pub struct Vars {
    browser: String,
    editor: String,
}

impl Vars {
    pub fn browser(&self) -> &str {
        &self.browser
    }

    pub fn editor(&self) -> &str {
        &self.editor
    }

    pub fn set_browser(&mut self, value: String) {
        self.browser = value;
    }

    pub fn set_editor(&mut self, value: String) {
        self.editor = value;
    }
}

impl Default for Vars {
    fn default() -> Vars {
        Vars {
            browser: "firefox".to_string(),
            editor: "vi".to_string(),
        }
    }
}

/// Start the editor to edit wydy variables.
pub fn edit_variables(vars: Vars) {
    let config_dir = config_dir();
    let env_file = config_dir.join("vars");
    match Command::new(vars.editor()).arg(env_file.display().to_string()).output() {
        Ok(_) => {}
        Err(e) => println!("Error on executing {}: {}", vars.editor(), e),
    }
}

use APP_INFO;
use app_dirs::{AppDataType, get_app_root};
use parser::parse_vars;
use std::fs::File;
use std::io::Read;
use std::process::Command;

#[derive(Debug)]
pub struct Var {
    name: String,
    value: String,
}

impl Var {
    pub fn new<S: Into<String>>(name: S, value: S) -> Var {
        Var {
            name: name.into(),
            value: value.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn set_value<S: Into<String>>(&mut self, new_value: S) {
        self.value = new_value.into();
    }
}

/// A structure that holds all variables.
pub struct Vars(Vec<Var>);

impl Vars {
    /// Load all vars from file
    pub fn load() -> Vars {
        let config_dir = get_app_root(AppDataType::UserConfig, &APP_INFO).unwrap();
        let path = config_dir.join("vars");
        let mut file = match File::open(&path) {
            Ok(f) => f,
            Err(_) => {
                File::create(&path).unwrap();
                return Vars::default();
            }
        };
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        Vars(parse_vars(content))
    }

    /// Search for the variable with this name and return is index.
    /// Return None if the value can't be find.
    fn index_of<S: Into<String>>(&self, name: S) -> Option<usize> {
        let name_l = name.into().to_lowercase();
        for (i, var) in self.0.iter().enumerate() {
            if name_l == var.name() {
                return Some(i);
            }
        }
        None
    }

    /// Return the value of the variable with this name.
    pub fn value_of<S: Into<String>>(&self, name: S) -> Option<String> {
        let idx = self.index_of(name);
        match idx {
            Some(i) => Some(self.0[i].value().to_string()),
            None => None,
        }
    }

    /// Set the value of the variable with this name.
    pub fn set_var(&mut self, name: &str, value: &str) {
        let idx = self.index_of(name);
        if let Some(i) = idx {
            self.0[i].set_value(value);
        }
    }
}

impl Default for Vars {
    fn default() -> Vars {
        let mut vars = Vec::new();
        vars.push(Var::new("browser", "firefox"));
        vars.push(Var::new("editor", "vi"));
        Vars(vars)
    }
}

/// Start the editor to edit wydy variables.
pub fn edit_variables(editor: String) {
    let config_dir = get_app_root(AppDataType::UserConfig, &APP_INFO).unwrap();
    let env_file = config_dir.join("vars");
    match Command::new(&editor).arg(env_file.display().to_string()).output() {
        Ok(_) => {}
        Err(e) => println!("Error on executing {}: {}", editor, e),
    }
}

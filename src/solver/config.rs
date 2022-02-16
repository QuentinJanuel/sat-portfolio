use std::sync::{Arc, Mutex};

use crate::cnf::Var;

#[derive(Clone)]
pub struct Config {
    kill: Arc<Mutex<bool>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            kill: Arc::new(Mutex::new(false)),
        }
    }
}

impl Config {
    pub fn get_kill(&self) -> bool {
        *self.kill.lock().unwrap()
    }
    /// Kills the solver, which will return as soon as possible
    /// Its result should not be used.
    pub fn kill(&self) {
        *self.kill.lock().unwrap() = true;
    }
    
    pub fn from_config_all(config_all: &ConfigAll) -> Self {
        config_all.config.clone()
    }
}

#[derive(Clone)]
pub struct ConfigAll<'a> {
    config: Config,
    interested_vars: Option<&'a Vec<Var>>,
}

impl Default for ConfigAll<'_> {
    fn default() -> Self {
        Self {
            config: Config::default(),
            interested_vars: None,
        }
    }
}

impl<'a> ConfigAll<'a> {
    /// Sets the variables that the solver is interested in.
    /// Two models will be considered equal even if they have different
    /// values for the variables the solver is not interested in.
    pub fn with_vars(&self, vars: &'a Vec<Var>) -> Self {
        Self {
            config: self.config.clone(),
            interested_vars: Some(vars),
        }
    }
    pub fn get_kill(&self) -> bool {
        self.config.get_kill()
    }
    /// Kills the solver, which will return as soon as possible
    /// Its result should not be used.
    pub fn kill(&self) {
        self.config.kill();
    }
    pub fn get_vars(&self) -> Option<&'a Vec<Var>> {
        self.interested_vars
    }
    pub fn from_config(config: &Config) -> Self {
        Self {
            config: config.clone(),
            ..Self::default()
        }
    }
}

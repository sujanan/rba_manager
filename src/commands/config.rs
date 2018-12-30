extern crate dirs;
extern crate serde;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::path::PathBuf;

use super::super::api::{ApiConfig, Credentials};

pub fn exec(args: &[String]) {
    let dirname = ".rba_manager";
    let filename = "config";
    let mut config_path = config_dir_path(dirname);

    if !config_path.exists() {
        fs::create_dir(config_path.as_path()).map_err(|error| {
            panic!(
                "error: creating '{}' failed: {}",
                config_path.display(),
                error
            );
        });
    }

    config_path.push(filename);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(config_path.as_path())
        .unwrap_or_else(|error| {
            panic!(
                "error: creating/opening '{}' failed: {}",
                config_path.display(),
                error
            );
        });
}

fn config_dir_path(dirname: &str) -> PathBuf {
    match dirs::home_dir() {
        Some(home) => {
            let mut buf = PathBuf::new();
            buf.push(home.as_path());
            buf.push(dirname);
            buf
        }
        None => {
            println!(
                "info: home directory is not available, current directory will be used instead."
            );
            match env::current_dir() {
                Ok(current) => current,
                Err(error) => panic!(
                    "error: current working directory value is invalid: {}",
                    error
                ),
            }
        }
    }
}

impl ApiConfig {
    fn credentials(&mut self, credentials: Option<Credentials>) -> &mut ApiConfig {
        self.credentials = credentials.or_else(|| {
            let mut username = prompt("username");
            let mut password = prompt("password");
            Some(Credentials { username, password })
        });
        self
    }

    fn url_params(&mut self, url_params: Option<Vec<String>>) -> &mut ApiConfig {
        self
    }
}

fn prompt(s: &str) -> String {
    let mut input = String::new();
    print!("{}: ", s);
    io::stdin()
        .read_line(&mut input)
        .expect("error: read_line failed");
    input
}

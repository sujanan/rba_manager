extern crate ctrlc;
extern crate dirs;
extern crate serde;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::path::PathBuf;

use super::super::api::*;

pub fn exec(args: &[String]) {
    let dirname = ".rba_manager";
    let filename = "config.toml";
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
        .append(true)
        .create(true)
        .open(config_path.as_path())
        .unwrap_or_else(|error| {
            panic!(
                "error: creating/opening '{}' failed: {}",
                config_path.display(),
                error
            );
        });

    toml_append_loop(&mut file);
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

fn build_config() -> ApiConfig {
    let mut config = ApiConfig::new();

    config
        .add(|| Endpoint {
            value: prompt("endpoint"),
        })
        .add(|| Credentials {
            username: prompt("username"),
            password: prompt("password"),
        })
        .add(|| ContentType {
            value: prompt("content-type"),
        })
        .add(|| Body {
            value: prompt("body"),
        })
        .add(|| StatusCode {
            value: u32_only_prompt("status_code"),
        });
    config
}

fn u32_only_prompt(s: &str) -> u32 {
    loop {
        return match prompt(s).parse::<u32>() {
            Ok(val) => val,
            Err(err) => {
                eprintln!("error: not a u32 value: {}", err);
                continue;
            }
        };
    }
}

fn toml_append_loop(toml: &mut fs::File) {
    let mut configs: HashMap<String, ApiConfig> = HashMap::new();
    loop {
        let name = prompt("name");
        banner(&name);
        configs.insert(name, build_config());
        let as_toml = toml::to_string(&configs).unwrap() + "\n";
        toml.write_all(as_toml.as_bytes());
    }
}

fn prompt(s: &str) -> String {
    let mut input = String::new();
    print!("{}: ", s);
    io::stdout().flush();
    io::stdin()
        .read_line(&mut input)
        .expect("error: read_line failed");
    input.pop();
    input
}

fn banner(s: &str) {
    println!("{:=<1$}", "", s.len());
    println!("{}", &s);
    println!("{:=<1$}", "", s.len());
    println!("");
}

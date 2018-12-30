#[macro_use]
extern crate serde_derive;
extern crate toml;

mod config;
mod help;

use std::env;

const ARGS: [(&str, &Fn(&[String])); 2] = [("config", &config::exec), ("help", &help::exec)];

fn main() {
	let args: Vec<String> = env::args().collect();	

	if args.len() == 1 {
		return;
	}
	let program = &args[1];

	for a in ARGS.iter() {
		if program == a.0 {
			(a.1)(&args[2..]);
		}	
	}
}

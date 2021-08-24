#![allow(dead_code)]
#[macro_use]
extern crate include_dir;
#[macro_use]
extern crate serde_derive;

use crate::cmd::new_pallet;
use crate::config::SubsConfig;
use anyhow::Result;
use std::path::PathBuf;

mod cli;
mod cmd;
mod config;
mod utils;

fn root_path() -> Result<PathBuf> {
	let mut path = dirs::home_dir().unwrap();
	path.push(".subs");
	Ok(path)
}

fn main() -> Result<()> {
	let matches = cli::build_cli().get_matches();

	let config_file = match matches.value_of("config") {
		Some(path) => PathBuf::from(path),
		None => root_path()?.join("config.toml"),
	};

	let res = match matches.subcommand() {
		("init", Some(matches)) => cmd::init(),
		("new", Some(matches)) => {
			let settings = SubsConfig::build(config_file).unwrap();
			let directory = matches.value_of("dir").unwrap_or_default();
			let input_option = SubsConfig::init_input(&settings, directory)?;
			new_pallet(directory, settings, input_option)
		}

		_ => unreachable!(),
	};

	if let Err(e) = res {
		log::error!("Error: {}", e);
		std::process::exit(101);
	}
	Ok(())
}

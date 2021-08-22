#[macro_use]
extern crate include_dir;

#[macro_use]
extern crate serde_derive;

use anyhow::Result;
use std::path::PathBuf;
use crate::config::SubsConfig;
use crate::cmd::new_node;

mod config;
mod cli;
mod cmd;
mod utils;


fn root_path() -> Result<PathBuf> {
    let mut path = dirs::home_dir().unwrap();
    path.push(".subs");
    Ok(path)
}

fn main() {
    let matches = cli::build_cli().get_matches();

    let config_file = match matches.value_of("config") {
        Some(path) => PathBuf::from(path),
        None => root_path().unwrap().join("config.toml"),
    };


	let res = match matches.subcommand() {
        ("init", Some(matches)) => {
            cmd::init()
        }
		("new", Some(matches)) => {
            let settings = SubsConfig::build(config_file).unwrap();

            let pallet = matches.value_of("pallet").unwrap_or_default();

            new_node(pallet)
        }

        _ => unreachable!(),
    };
}

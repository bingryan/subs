mod config;
mod cli;

#[macro_use]
extern crate include_dir;

use include_dir::Dir;

static PROJECT_NODE_DIR: Dir = include_dir!("templates/node");
static PROJECT_DIR: Dir = include_dir!("templates/");

fn main() {
	let matches = cli::build_cli().get_matches();

	let res = match matches.subcommand() {
		("new", Some(matches)) => {
			let pallet = matches.value_of("pallet").unwrap_or_default();

			println!("{:?}", pallet);

		}
		_ => unreachable!(),
	};
}

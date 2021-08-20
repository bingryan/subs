#[macro_use]
extern crate include_dir;

use include_dir::Dir;

static PROJECT_NODE_DIR: Dir = include_dir!("templates/node");
static PROJECT_DIR: Dir = include_dir!("templates/");

fn main() {
	let glob = "*";
	for entry in PROJECT_NODE_DIR.find(glob).unwrap() {
		println!("node: Found {}", entry.path().display());
	}

	for entry in PROJECT_DIR.find(glob).unwrap() {
		println!("Found {}", entry.path().display());
	}
}

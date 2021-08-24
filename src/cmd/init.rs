use anyhow::{Error, Result};
use console::{style, Emoji};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use crate::config::{CONFIG, INIT_TEMPLATE_REGISTER, LOOKING_GLASS, SPARKLE};
use crate::root_path;
use crate::utils::file::{create_file, is_empty_directory};
use git2::build::RepoBuilder;
use std::env;
use std::fs::create_dir_all;

pub fn init() -> Result<()> {
	let started = Instant::now();

	let step_size = 2;

	let path = root_path()?;
	println!(
		"{} {}Create subs config file at : {}",
		style(format!("[1/{}]", step_size)).bold().dim(),
		LOOKING_GLASS,
		&path.join("config.toml").display()
	);

	let config = CONFIG.trim_start();

	create_file(&path.join("config.toml"), &config)?;

	let template_registry = root_path()?.join("templates");

	println!(
		"{} {}git pull substrate template at : {}",
		style(format!("[2/{}]", step_size)).bold().dim(),
		LOOKING_GLASS,
		&template_registry.display()
	);

	RepoBuilder::new()
		.clone(INIT_TEMPLATE_REGISTER, template_registry.as_path())
		.expect(format!("IO error: git pull {}", INIT_TEMPLATE_REGISTER).as_ref());

	println!("{} Done in {}", SPARKLE, HumanDuration(started.elapsed()));
	Ok(())
}

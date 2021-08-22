use anyhow::{Error, Result};
use console::{style, Emoji};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use crate::config::{LOOKING_GLASS, SPARKLE, CONFIG};
use crate::root_path;
use crate::utils::file::{create_file, is_empty_directory};
use std::fs::{create_dir_all};
use std::env;


pub fn init() -> Result<()> {
    let started = Instant::now();

    let step_size = 1;

    let path = root_path()?;
    println!(
        "{} {}Create subs config file at : {}",
        style(format!("[1/{}]", step_size)).bold().dim(),
        LOOKING_GLASS,
        &path.join("config.yml").display()
    );


    let config = CONFIG.trim_start();

    create_file(&path.join("config.yml"), &config)?;

    println!("{} Done in {}", SPARKLE, HumanDuration(started.elapsed()));
    Ok(())
}

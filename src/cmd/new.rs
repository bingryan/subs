use crate::config::{NewInputOptions, SubsConfig};
use crate::root_path;
use crate::utils::file::{copy_than_render, create_file};
use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::str;
use std::{env, fs};
use walkdir::WalkDir;

pub fn new_pallet(directory: &str, settings: SubsConfig, input_option: NewInputOptions) -> Result<()> {
	let target_pallet_path = env::current_dir()?.join(directory);
	let template_path = root_path()?.join(settings.system.templates);
	if input_option.is_parachain() {
		copy_than_render(
			template_path.join("parachain"),
			target_pallet_path,
			&input_option.hashmap(),
		)?;
	} else {
		copy_than_render(template_path.join("node"), target_pallet_path, &input_option.hashmap())?;
	}
	Ok(())
}

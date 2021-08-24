use anyhow::Result;
use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::{fs, str};

pub fn create_file<T: AsRef<Path>>(path: T, content: &str) -> Result<()> {
	let target_path = PathBuf::from(path.as_ref());
	if let Some(p) = target_path.parent() {
		create_dir_all(p)?;
	}
	let mut file = File::create(&target_path)?;
	file.write_all(content.as_bytes())?;
	Ok(())
}

pub fn is_empty_directory<T: AsRef<Path>>(path: T) -> Result<bool> {
	let target_path = PathBuf::from(path.as_ref());
	if target_path.is_dir() {
		let is_entry = match target_path.read_dir() {
			Ok(_entries) => Ok(false),
			Err(_e) => Ok(true),
		};
		return is_entry;
	}
	Ok(false)
}

pub fn copy_than_render<O: AsRef<Path>, T: AsRef<Path>>(
	from: O,
	to: T,
	options: &HashMap<String, String>,
) -> Result<()> {
	let mut stack = Vec::new();
	stack.push(PathBuf::from(from.as_ref()));

	let output_root = PathBuf::from(to.as_ref());
	let input_root = PathBuf::from(from.as_ref()).components().count();

	while let Some(working_path) = stack.pop() {
		let src: PathBuf = working_path.components().skip(input_root).collect();

		let dest = if src.components().count() == 0 {
			output_root.clone()
		} else {
			output_root.join(&src)
		};
		if fs::metadata(&dest).is_err() {
			fs::create_dir_all(&dest)?;
		}

		for entry in fs::read_dir(working_path)? {
			let entry = entry?;
			let path = entry.path();
			if path.is_dir() {
				stack.push(path);
			} else {
				match path.file_name() {
					Some(filename) => {
						let template = mustache::compile_path(&path).expect("Failed to compile");
						let mut bytes = vec![];
						template.render(&mut bytes, options).expect("Failed to render");
						let content = str::from_utf8(&bytes).unwrap();
						let dest_path = dest.join(filename);
						create_file(&dest_path, content)?;
					}
					None => {
						log::error!("Subs Error: failed with {:?}", path);
					}
				}
			}
		}
	}

	Ok(())
}

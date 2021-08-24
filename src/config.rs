use crate::utils::uppercase_first_letter;
use anyhow::Result;
use config::{Config, ConfigError, File};
use console::{Emoji, Style};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use include_dir::Dir;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::result;

pub const CONFIG: &str = r#"[node]
name = "template"
license = "Unlicense"
authors = "Genius team"
version = "4.0.0-dev"
tag = "monthly-2021-08"

[parachain]
name = "template"
license = "Unlicense"
authors = "Genius team"
branch = "polkadot-v0.9.8"

[system]
templates = "templates"
repository = "https://github.com/bingryan/substrate-template.git"
"#;

pub static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
pub static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
pub static CLIP: Emoji<'_, '_> = Emoji("🔗  ", "");
pub static PAPER: Emoji<'_, '_> = Emoji("📃  ", "");
pub static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");

pub static INIT_TEMPLATE_REGISTER: &str = "https://github.com/bingryan/substrate-template.git";

#[derive(Debug, Deserialize, Clone)]
pub struct NodeInfo {
	pub name: String,
	pub license: String,
	pub authors: String,
	pub version: String,
	pub tag: String,
}

impl Default for NodeInfo {
	fn default() -> Self {
		NodeInfo {
			name: "template".to_string(),
			license: "Unlicense".to_string(),
			authors: "Genius team".to_string(),
			version: "4.0.0-dev".to_string(),
			tag: "monthly-2021-08".to_string(),
		}
	}
}

#[derive(Debug, Deserialize, Clone)]
pub struct ParaInfo {
	pub name: String,
	pub license: String,
	pub authors: String,
	pub branch: String,
}

impl Default for ParaInfo {
	fn default() -> Self {
		ParaInfo {
			name: "template".to_string(),
			license: "Unlicense".to_string(),
			authors: "Genius team".to_string(),
			branch: "polkadot-v0.9.8".to_string(),
		}
	}
}

#[derive(Debug, Deserialize, Clone)]
pub struct System {
	pub templates: String,
}

impl Default for System {
	fn default() -> Self {
		System {
			templates: "./templates".to_string(),
		}
	}
}

#[derive(Debug, Deserialize, Clone)]
pub struct SubsConfig {
	pub node: NodeInfo,
	pub parachain: ParaInfo,
	pub system: System,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NewInputOptions {
	pub name: String,
	pub description: String,
	pub module: String,
	pub license: String,
	pub authors: String,
	pub version: String,
	pub tag: String,
	pub branch: String,
	pub is_node: bool,
}

impl NewInputOptions {
	pub fn hashmap(&self) -> HashMap<String, String> {
		let mut data = HashMap::new();
		data.insert("name".to_string(), self.name.to_string());
		data.insert("description".to_string(), self.description.to_string());
		data.insert("license".to_string(), self.license.to_string());
		data.insert("authors".to_string(), self.authors.to_string());
		data.insert("version".to_string(), self.version.to_string());
		data.insert("tag".to_string(), self.tag.to_string());
		data.insert("branch".to_string(), self.branch.to_string());
		data.insert("module".to_string(), self.module.to_string());
		data
	}
	pub fn is_parachain(&self) -> bool {
		!self.is_node
	}
}

impl SubsConfig {
	pub fn build(file: PathBuf) -> result::Result<Self, ConfigError> {
		let mut s = Config::new();
		s.merge(File::from(file))?;
		s.try_into()
	}

	pub fn init_input(config: &SubsConfig, directory: &str) -> Result<NewInputOptions> {
		let theme = ColorfulTheme {
			values_style: Style::new().blue().dim(),
			..ColorfulTheme::default()
		};
		println!(
			"{}Crate a new pallet at: {}",
			TRUCK,
			env::current_dir()?.join(directory).display()
		);

		let template_type = Select::with_theme(&theme)
			.with_prompt("Configure pallet template type")
			.default(0)
			.item("node")
			.item("parachain")
			.interact()?;

		let name = Input::with_theme(&theme)
			.with_prompt("name")
			.default("template".to_string())
			.interact()?;

		let description = Input::with_theme(&theme)
			.with_prompt("description")
			.default("awesome project !".to_string())
			.interact()?;

		let module = uppercase_first_letter(name.to_string());

		let license = Input::with_theme(&theme)
			.with_prompt("license")
			.default("Unlicense".to_string())
			.interact()?;

		let authors = Input::with_theme(&theme)
			.with_prompt("authors")
			.default("Genius Team".to_string())
			.interact()?;

		let (branch, version, tag, is_node) = match template_type {
			0 => {
				let version = Input::with_theme(&theme)
					.with_prompt("version")
					.default("4.0.0-dev".to_string())
					.interact()?;

				let tag = Input::with_theme(&theme)
					.with_prompt("tag")
					.default("monthly-2021-08".to_string())
					.interact()?;
				("branch".to_lowercase(), version, tag, true)
			}
			1 => {
				let branch = Input::with_theme(&theme)
					.with_prompt("branch")
					.default("polkadot-v0.9.8".to_string())
					.interact()?;

				(
					branch.to_lowercase(),
					"version".to_lowercase(),
					"tag".to_lowercase(),
					false,
				)
			}
			_ => (
				"branch".to_lowercase(),
				"version".to_lowercase(),
				"tag".to_lowercase(),
				true,
			),
		};

		Ok(NewInputOptions {
			name,
			description,
			module,
			license,
			authors,
			version,
			tag,
			branch,
			is_node,
		})
	}
}

use config::{Config, ConfigError, File};
use console::{Emoji, Style};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use include_dir::Dir;
use std::error::Error;
use std::path::PathBuf;
use std::result;
use std::env;
use anyhow::Result;

pub const CONFIG: &str = r#"[node]
name = "Genius team"
license = "Unlicense"
authors = "Genius team"
version = "4.0.0-dev"
tag = "monthly-2021-08"

[para]
name = "Genius team"
license = "Unlicense"
authors = "Genius team"
branch = "polkadot-v0.9.8"
"#;
pub static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
pub static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
pub static CLIP: Emoji<'_, '_> = Emoji("🔗  ", "");
pub static PAPER: Emoji<'_, '_> = Emoji("📃  ", "");
pub static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");

static TEMPLATE_NODE: Dir = include_dir!("templates/node");

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
            name: "Genius team".to_string(),
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
            name: "Genius team".to_string(),
            license: "Unlicense".to_string(),
            authors: "Genius team".to_string(),
            branch: "polkadot-v0.9.8".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct SubsConfig {
    pub node: NodeInfo,
    pub parachain: ParaInfo,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NewInputOptions {
    pub name: String,
    pub license: String,
    pub authors: String,
    pub version: String,
    pub tag: String,
    pub branch: String,
    pub is_node: bool,
}

impl SubsConfig {
    pub fn build(file: PathBuf) -> result::Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name(file.into_os_string().into_string().unwrap().as_str()))?;
        s.try_into()
    }

    pub fn init_input(directory: &str) -> Result<NewInputOptions> {
        let theme = ColorfulTheme {
            values_style: Style::new().blue().dim(),
            ..ColorfulTheme::default()
        };

        println!("{}Crate a new pallet at: {}", TRUCK, env::current_dir()?.join(directory).display());

        let template_type = Select::with_theme(&theme)
            .with_prompt("Configure pallet template type")
            .default(0)
            .item("node")
            .item("parachain")
            .interact()?;

        let name = Input::with_theme(&theme)
            .with_prompt("name")
            .default("Anonymous".to_string())
            .interact()?;

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
                ("branch".to_lowercase(), version, tag, false)
            }
            1 => {
                let branch = Input::with_theme(&theme)
                    .with_prompt("branch")
                    .default("polkadot-v0.9.8".to_string())
                    .interact()?;

                (branch.to_lowercase(), "version".to_lowercase(), "tag".to_lowercase(), true)
            }
            _ => ("branch".to_lowercase(), "version".to_lowercase(), "tag".to_lowercase(), true),
        };

        Ok(NewInputOptions {
            name,
            license,
            authors,
            version,
            tag,
            branch,
            is_node,
        })
    }
}

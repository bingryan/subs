use include_dir::Dir;
use std::path::PathBuf;
use std::result;
use config::{ConfigError, Config, File};
use console::Emoji;

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

impl SubsConfig {
    pub fn build(file: PathBuf) -> result::Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name(file.into_os_string().into_string().unwrap().as_str()))?;
        s.try_into()
    }
}
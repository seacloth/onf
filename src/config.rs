use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct ProfileEntry {
    pub original: String,
    pub alias: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ProfileHooks {
    pub post_apply: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub active: Option<String>,
    pub profiles: HashMap<String, Vec<ProfileEntry>>,
    #[serde(default)]
    pub hooks: HashMap<String, ProfileHooks>,
}

pub fn config_dir() -> PathBuf {
    dirs::config_dir()
        .expect("could not find config directory")
        .join("onf")
}

pub fn profiles_dir() -> PathBuf {
    config_dir().join("profiles")
}

pub fn config_path() -> PathBuf {
    config_dir().join("config.toml")
}

pub fn load() -> anyhow::Result<Config> {
    let path = config_path();
    if !path.exists() {
        return Ok(Config::default());
    }
    let text = fs::read_to_string(&path)?;
    Ok(toml::from_str(&text)?)
}

pub fn save(cfg: &Config) -> anyhow::Result<()> {
    let dir = config_dir();
    fs::create_dir_all(&dir)?;
    let text = toml::to_string_pretty(cfg)?;
    fs::write(config_path(), text)?;
    Ok(())
}

use eyre::WrapErr;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModelSettings {
    pub output_home: Option<PathBuf>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct BurntAppleConfig {
    pub models: BTreeMap<String, ModelSettings>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DaemonMetadata {
    pub model_id: String,
    pub pid: u32,
    pub port: u16,
}

impl BurntAppleConfig {
    pub fn load() -> eyre::Result<Self> {
        let path = config_file_path();
        if !path.exists() {
            return Ok(Self::default());
        }

        let bytes = fs::read(&path)
            .wrap_err_with(|| format!("Failed to read config file at {}", path.display()))?;
        let config = serde_json::from_slice(&bytes)
            .wrap_err_with(|| format!("Failed to parse config file at {}", path.display()))?;
        Ok(config)
    }

    pub fn save(&self) -> eyre::Result<()> {
        let path = config_file_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .wrap_err_with(|| format!("Failed to create config directory {}", parent.display()))?;
        }

        let body = serde_json::to_vec_pretty(self).wrap_err("Failed to serialize config")?;
        fs::write(&path, body)
            .wrap_err_with(|| format!("Failed to write config file at {}", path.display()))?;
        Ok(())
    }

    pub fn model_settings_mut(&mut self, model_id: &str) -> &mut ModelSettings {
        self.models.entry(model_id.to_string()).or_default()
    }

    #[must_use]
    pub fn model_output_home(&self, model_id: &str) -> Option<PathBuf> {
        self.models.get(model_id).and_then(|model| model.output_home.clone())
    }
}

#[must_use]
pub fn daemon_pid_path_for_model(model_id: &str) -> PathBuf {
    app_state_dir().join("daemon").join(format!("{}.pid", slugify_model_id(model_id)))
}

#[must_use]
pub fn daemon_metadata_path_for_model(model_id: &str) -> PathBuf {
    app_state_dir()
        .join("daemon")
        .join(format!("{}.json", slugify_model_id(model_id)))
}

pub fn save_daemon_metadata(model_id: &str, metadata: &DaemonMetadata) -> eyre::Result<()> {
    let path = daemon_metadata_path_for_model(model_id);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .wrap_err_with(|| format!("Failed to create daemon directory {}", parent.display()))?;
    }

    let body = serde_json::to_vec_pretty(metadata).wrap_err("Failed to serialize daemon metadata")?;
    fs::write(&path, body)
        .wrap_err_with(|| format!("Failed to write daemon metadata at {}", path.display()))?;
    Ok(())
}

pub fn load_daemon_metadata(model_id: &str) -> eyre::Result<Option<DaemonMetadata>> {
    let path = daemon_metadata_path_for_model(model_id);
    if !path.exists() {
        return Ok(None);
    }

    let bytes = fs::read(&path)
        .wrap_err_with(|| format!("Failed to read daemon metadata at {}", path.display()))?;
    let metadata = serde_json::from_slice(&bytes)
        .wrap_err_with(|| format!("Failed to parse daemon metadata at {}", path.display()))?;
    Ok(Some(metadata))
}

pub fn clear_daemon_metadata(model_id: &str) -> eyre::Result<()> {
    let pid_path = daemon_pid_path_for_model(model_id);
    if pid_path.exists() {
        fs::remove_file(&pid_path)
            .wrap_err_with(|| format!("Failed to remove pid file {}", pid_path.display()))?;
    }

    let metadata_path = daemon_metadata_path_for_model(model_id);
    if metadata_path.exists() {
        fs::remove_file(&metadata_path).wrap_err_with(|| {
            format!(
                "Failed to remove daemon metadata file {}",
                metadata_path.display()
            )
        })?;
    }

    Ok(())
}

#[must_use]
fn config_file_path() -> PathBuf {
    app_state_dir().join("config.json")
}

#[must_use]
fn app_state_dir() -> PathBuf {
    if cfg!(windows) {
        if let Some(appdata) = env::var_os("APPDATA") {
            return PathBuf::from(appdata).join("burnt-apple");
        }
    }

    if let Some(xdg_config_home) = env::var_os("XDG_CONFIG_HOME") {
        return PathBuf::from(xdg_config_home).join("burnt-apple");
    }

    if let Some(home) = env::var_os("HOME") {
        return PathBuf::from(home).join(".config").join("burnt-apple");
    }

    Path::new(".").join(".burnt-apple")
}

#[must_use]
fn slugify_model_id(model_id: &str) -> String {
    model_id
        .chars()
        .map(|character| match character {
            '/' | '\\' | ':' | ' ' => '_',
            other => other,
        })
        .collect()
}

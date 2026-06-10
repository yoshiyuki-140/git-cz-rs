use anyhow::{Context, Result};
use dirs::home_dir;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::entity::CzConfig;

const CONFIG_FILENAME: &str = "cz.json";

/// cz.jsonを読み込む関数
pub fn load_config() -> Result<Option<CzConfig>> {
    let mut config_path = find_config_upwards(CONFIG_FILENAME);

    if config_path.is_none()
        && let Some(mut home) = home_dir()
    {
        home.push(CONFIG_FILENAME);
        if home.is_file() {
            config_path = Some(home);
        }
    }

    if let Some(path) = config_path {
        let json_str = fs::read_to_string(&path)
            .with_context(|| format!("{:?}の読み込みに失敗しました", path))?;
        let config: CzConfig =
            serde_json::from_str(&json_str).context("cz.jsonの形式が正しくありません")?;
        Ok(Some(config))
    } else {
        Ok(None)
    }
}

/// 親ディレクトリ方向にCONFIG_FILENAMEを探索する関数
fn find_config_upwards(filename: &str) -> Option<PathBuf> {
    let mut current_dir = env::current_dir().ok()?;
    loop {
        let config_path = current_dir.join(filename);
        if config_path.is_file() {
            return Some(config_path);
        }
        if !current_dir.pop() {
            break;
        }
    }
    None
}

/// gitコマンドを実行する関数
pub fn execute_git_commit(message: &str) -> Result<bool> {
    let status = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(message)
        .status()?;
    Ok(status.success())
}

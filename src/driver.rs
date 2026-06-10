use anyhow::{Context, Result};
use dirs::home_dir;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::entity::CzConfig;

/// 探索する設定ファイル名
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
        Ok(Some(parse_config(&path)?))
    } else {
        Ok(None)
    }
}

/// 指定パスの JSON ファイルを読み込み CzConfig としてパースする
fn parse_config(path: &PathBuf) -> Result<CzConfig> {
    let json_str = fs::read_to_string(path)
        .with_context(|| format!("{:?}の読み込みに失敗しました", path))?;
    serde_json::from_str(&json_str).context("cz.jsonの形式が正しくありません")
}

/// 親ディレクトリ方向にCONFIG_FILENAMEを探索する関数
fn find_config_upwards(filename: &str) -> Option<PathBuf> {
    find_config_from(filename, env::current_dir().ok()?)
}

/// start を起点として親ディレクトリ方向に filename を探索する
fn find_config_from(filename: &str, start: PathBuf) -> Option<PathBuf> {
    let mut current_dir = start;
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    /// 起点ディレクトリに cz.json がある場合、そのパスを返すこと
    #[test]
    fn test_find_config_from_finds_file() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("cz.json");
        fs::write(&config_path, "{}").unwrap();
        let result = find_config_from("cz.json", dir.path().to_path_buf());
        assert_eq!(result, Some(config_path));
    }

    /// cz.json が存在しないディレクトリを起点とした場合、None を返すこと
    #[test]
    fn test_find_config_from_returns_none_when_missing() {
        let dir = tempdir().unwrap();
        let result = find_config_from("cz.json", dir.path().to_path_buf());
        assert!(result.is_none());
    }

    /// 有効な JSON ファイルを読み込んだ場合、CzConfig として正しくパースされること
    #[test]
    fn test_parse_config_valid_json() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("cz.json");
        fs::write(&config_path, r#"{"options": {"feat": "新機能"}}"#).unwrap();
        let result = parse_config(&config_path).unwrap();
        assert_eq!(result.options["feat"], "新機能");
    }
}

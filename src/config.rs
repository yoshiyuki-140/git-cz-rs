use anyhow::{Context, Ok, Result};
use dirs::home_dir;
use indexmap::IndexMap;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;

// 探すファイル名
const CONFIG_FILENAME: &str = "cz.json";

// デフォルトのハードコード
pub const DEFAULT_PROMPT_OPTIONS: [&str; 8] = [
    "feat:      新機能",
    "fix:       バグ修正",
    "docs:      ドキュメントのみの変更",
    "style:     コードの意味に影響を与えない変更",
    "refactor:  バグ修正も新機能追加も行わないコード変更",
    "perf:      パフォーマンス向上",
    "test:      テストの追加・修正",
    "chore:     ビルドプロセスやツールの変更",
];

// 設定ファイルを紐づけるための型
#[derive(Deserialize, Debug)]
pub struct CzConfig {
    pub options: IndexMap<String, String>,
    pub scopes: Option<Vec<String>>,
}

// cz.jsonの設定を読み込む関数
pub fn load_config() -> Result<Option<CzConfig>> {
    let mut config_path = find_config_upwards(CONFIG_FILENAME);

    // 設定ファイルがない場合
    if config_path.is_none() {
        if let Some(mut home) = home_dir() {
            home.push(CONFIG_FILENAME);
            if home.is_file() {
                config_path = Some(home);
            }
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

// cz.jsonを親ディレクトリ方向に探索する関数
// Option型はデータがNoneになる可能性がある場合に使うため、引数の型はOption型になっている
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

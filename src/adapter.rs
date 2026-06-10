use anyhow::Result;
use inquire::{Select, Text};

use crate::entity::CzConfig;

/// コミット種別を選択する関数
pub fn select_commit_type(options: Vec<String>) -> Result<String> {
    let selection = Select::new("コミットタイプを選択してください", options).prompt()?;
    let commit_type = selection.split(':').next().unwrap().trim().to_string();
    Ok(commit_type)
}

/// スコープを選択する関数
pub fn select_or_input_scope(config: &Option<CzConfig>) -> Result<String> {
    if let Some(Some(scopes)) = config.as_ref().map(|c| c.scopes.as_ref()) {
        let mut scope_options = vec!["(スキップ)".to_string()];
        scope_options.extend(scopes.clone());
        let selected = Select::new("スコープを選択してください", scope_options).prompt()?;
        if selected == "(スキップ)" {
            Ok(String::new())
        } else {
            Ok(format!("({selected})"))
        }
    } else {
        let scope =
            Text::new("スコープを入力してください(例: ui, parser) [Enterでスキップ]:").prompt()?;
        if scope.is_empty() {
            Ok(String::new())
        } else {
            Ok(format!("({scope})"))
        }
    }
}

/// 変更内容の要約を取得する関数
pub fn input_subject() -> Result<String> {
    Ok(Text::new("変更内容の要約を入力してください:").prompt()?)
}

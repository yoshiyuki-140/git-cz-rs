// ユーザとのやり取りをする関数をまとている
use anyhow::Result;
use inquire::{Select, Text};

use crate::entity::CzConfig;
use crate::usecase::resolve_prompt_options;

/// コミット種別を選択する関数
pub fn select_commit_type(config: &Option<CzConfig>) -> Result<String> {
    let options = resolve_prompt_options(config);
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
pub fn input_subject(config: &Option<CzConfig>) -> Result<String> {
    loop {
        let subject = Text::new("変更内容の要約を入力してください:").prompt()?;
        if let Some(max_commit_message_size) = config
            .as_ref()
            .and_then(|c| c.subject.as_ref())
            .and_then(|s| s.max_commit_message_size)
            && subject.chars().count() > max_commit_message_size
        {
            eprintln!(
                "変更内容の要約は {} 文字以内にしてください",
                max_commit_message_size
            );
            continue;
        }
        print!("{}文字", subject.chars().count()); // 制限文字数以下の場合に文字数を出力する
        return Ok(subject);
    }
}

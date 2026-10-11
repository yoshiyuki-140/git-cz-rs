use entity::{CommitMessage, CzConfig, DEFAULT_PROMPT_OPTIONS};

/// config の有無に応じてプロンプト用の選択肢一覧を返す
pub fn resolve_prompt_options(config: &Option<CzConfig>) -> Vec<String> {
    match config {
        Some(c) => c
            .commit_types
            .iter()
            .map(|(k, v)| format!("{k}: {v}"))
            .collect(),
        None => DEFAULT_PROMPT_OPTIONS
            .iter()
            .map(|s| s.to_string())
            .collect(),
    }
}

/// type・scope・subject から CommitMessage を組み立てる
pub fn build_commit_message(
    commit_type: String,
    scope: String,
    description: String,
    body: String,
    footer: String,
) -> String {
    let commit_message = CommitMessage::new(commit_type, scope, description, body, footer);
    format!(
        "{}{}: {}\n\n{}\n\n{}",
        commit_message.commit_type,
        commit_message.scope,
        commit_message.description,
        commit_message.body,
        commit_message.footer
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;

    /// config が None の場合、DEFAULT_PROMPT_OPTIONS の件数と先頭要素が返ること
    #[test]
    fn test_resolve_options_returns_defaults_when_no_config() {
        let options = resolve_prompt_options(&None);
        assert_eq!(options.len(), DEFAULT_PROMPT_OPTIONS.len());
        assert!(options[0].starts_with("feat"));
    }

    /// config が Some の場合、その options を "key: value" 形式で返すこと
    #[test]
    fn test_resolve_options_uses_config_options() {
        let mut map = IndexMap::new();
        map.insert("custom".to_string(), "カスタム変更".to_string());
        let config = Some(CzConfig {
            commit_types: map,
            scopes: None,
            description: None,
        });
        let options = resolve_prompt_options(&config);
        assert_eq!(options, vec!["custom: カスタム変更"]);
    }

    /// type・scope・subject を渡すと正しくフォーマットされた CommitMessage が返ること
    #[test]
    fn test_build_commit_message() {
        let msg = build_commit_message(
            "feat".to_string(),
            "(api)".to_string(),
            "エンドポイントを追加".to_string(),
            "エンドポイントを追加しました".to_string(),
            "BREAKING CHANGE:".to_string(),
        );
        assert_eq!(
            msg,
            "feat(api): エンドポイントを追加\n\nエンドポイントを追加しました\n\nBREAKING CHANGE:"
        );
    }

    /// スコープなしの場合、"type: subject" 形式でフォーマットされること
    #[test]
    fn test_format_without_scope() {
        let msg = build_commit_message(
            "fix".to_string(),
            String::new(),
            "バグ修正".to_string(),
            "".to_string(),
            "".to_string(),
        );
        assert_eq!(msg, "fix: バグ修正\n\n\n\n");
    }
}

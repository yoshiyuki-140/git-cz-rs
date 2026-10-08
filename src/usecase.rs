use crate::entity::{CommitMessage, CzConfig, DEFAULT_PROMPT_OPTIONS};

/// config の有無に応じてプロンプト用の選択肢一覧を返す
pub fn resolve_prompt_options(config: &Option<CzConfig>) -> Vec<String> {
    match config {
        Some(c) => c.options.iter().map(|(k, v)| format!("{k}: {v}")).collect(),
        None => DEFAULT_PROMPT_OPTIONS
            .iter()
            .map(|s| s.to_string())
            .collect(),
    }
}

/// type・scope・subject から CommitMessage を組み立てる
pub fn build_commit_message(commit_type: String, scope: String, subject: String) -> String {
    let commit_message = CommitMessage::new(commit_type, scope, subject);
    format!(
        "{}{}: {}",
        commit_message.commit_type, commit_message.scope, commit_message.subject
    )
}

#[cfg(test)]
mod tests {
    use crate::entity::SubjectConfig;

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
            options: map,
            scopes: None,
            subject: SubjectConfig {
                max_commit_message_size: Some(50),
            },
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
        );
        assert_eq!(msg, "feat(api): エンドポイントを追加");
    }
    /// スコープありの場合、"type(scope): subject" 形式でフォーマットされること
    #[test]
    fn test_format_with_scope() {
        let msg = build_commit_message(
            "feat".to_string(),
            "(ui)".to_string(),
            "ボタンを追加".to_string(),
        );
        assert_eq!(msg, "feat(ui): ボタンを追加");
    }

    /// スコープなしの場合、"type: subject" 形式でフォーマットされること
    #[test]
    fn test_format_without_scope() {
        let msg = build_commit_message("fix".to_string(), String::new(), "バグ修正".to_string());
        assert_eq!(msg, "fix: バグ修正");
    }
}

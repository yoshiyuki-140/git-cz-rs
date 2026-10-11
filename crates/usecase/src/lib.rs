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
    breaking_change_message: String,
) -> String {
    let commit_message = CommitMessage::new(
        commit_type,
        scope,
        description,
        body,
        footer,
        breaking_change_message,
    );

    // 破壊的変更がある場合
    // 型・スコープの接尾辞に'!'記号と'BREAKING CHANGE:'及びコメントを結合する
    // 参考：https://www.conventionalcommits.org/ja/v1.0.0/#-%E3%81%A8-breaking-change-%E3%83%95%E3%83%83%E3%82%BF%E3%83%BC%E3%81%AE%E4%B8%A1%E6%96%B9%E3%82%92%E6%8C%81%E3%81%A4%E3%82%B3%E3%83%9F%E3%83%83%E3%83%88%E3%83%A1%E3%83%83%E3%82%BB%E3%83%BC%E3%82%B8
    if commit_message.breaking_change_message.is_empty() {
        format!(
            "{}{}: {}\n\n{}\n\n{}",
            commit_message.commit_type,
            commit_message.scope,
            commit_message.description,
            commit_message.body,
            commit_message.footer,
        )
    } else {
        format!(
            "{}{}!: {}\n\n{}\n\n{}\nBREAKING CHANGE: {}",
            commit_message.commit_type,
            commit_message.scope,
            commit_message.description,
            commit_message.body,
            commit_message.footer,
            commit_message.breaking_change_message
        )
    }
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

    /// type・scope・descriptionを渡すと正しくフォーマットされた CommitMessage が返ること
    #[test]
    fn test_build_commit_message() {
        let msg = build_commit_message(
            "feat".to_string(),
            "(api)".to_string(),
            "エンドポイントを追加".to_string(),
            "エンドポイントを追加しました".to_string(),
            "".to_string(),
            "".to_string(),
        );
        assert_eq!(
            msg,
            "feat(api): エンドポイントを追加\n\nエンドポイントを追加しました\n\n"
        );
    }
}

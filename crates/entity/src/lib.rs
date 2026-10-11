use indexmap::IndexMap;
use serde::Deserialize;

/// cz.json が存在しない場合に使用するデフォルトのコミットタイプ一覧
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

/// cz.jsonの形式を指定してる
#[derive(Deserialize, Debug)]
pub struct CzConfig {
    pub commit_types: IndexMap<String, String>,
    // Optionを付けるのはcz.jsonに存在無くてもエラーを回避するため
    pub scopes: Option<Vec<String>>,
    pub description: Option<SubjectConfig>,
}
// コミットメッセージの主題テキストデータに課すデータ構造
#[derive(Deserialize, Debug)]
pub struct SubjectConfig {
    pub max_commit_message_size: Option<usize>,
}

/// コミットメッセージの型を指定している
pub struct CommitMessage {
    pub commit_type: String,
    pub scope: String,
    pub description: String,
    pub body: String,
    pub footer: String,
    pub breaking_change_message: String, // ここが""(空文字)の時は、ブレイキングチェンジとしない
}

impl CommitMessage {
    /// CommitMessageのコンストラクタ
    pub fn new(
        commit_type: String,
        scope: String,
        description: String,
        body: String,
        footer: String,
        breaking_change_message: String,
    ) -> Self {
        Self {
            commit_type,
            scope,
            description,
            body,
            footer,
            breaking_change_message,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // データ構造が作成できること
    #[test]
    fn test_create_commit_message_data() {
        let msg = CommitMessage::new(
            "feat".to_string(),
            "(ui)".to_string(),
            "ボタンを追加".to_string(),
            "sample body".to_string(),
            "sample footer".to_string(),
            "".to_string(),
        );
        assert_eq!(msg.commit_type, "feat");
        assert_eq!(msg.scope, "(ui)");
        assert_eq!(msg.description, "ボタンを追加");
        assert_eq!(msg.body, "sample body");
        assert_eq!(msg.footer, "sample footer");
        assert_eq!(msg.breaking_change_message, "");
    }
}

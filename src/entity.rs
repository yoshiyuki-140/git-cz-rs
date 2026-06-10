/// これらは変更があまりされないだろうからentityに含めました
use indexmap::IndexMap;
use serde::Deserialize;

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
    pub options: IndexMap<String, String>,
    pub scopes: Option<Vec<String>>,
}

/// コミットメッセージの型を指定している
pub struct CommitMessage {
    pub commit_type: String,
    pub scope: String,
    pub subject: String,
}

impl CommitMessage {
    /// CommitMessageのコンストラクタ
    pub fn new(commit_type: String, scope: String, subject: String) -> Self {
        Self {
            commit_type,
            scope,
            subject,
        }
    }

    /// コミットメッセージを成形
    pub fn format(&self) -> String {
        format!("{}{}: {}", self.commit_type, self.scope, self.subject)
    }
}

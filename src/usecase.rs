use crate::entity::{CommitMessage, CzConfig, DEFAULT_PROMPT_OPTIONS};

pub fn resolve_prompt_options(config: &Option<CzConfig>) -> Vec<String> {
    match config {
        Some(c) => c.options.iter().map(|(k, v)| format!("{k}: {v}")).collect(),
        None => DEFAULT_PROMPT_OPTIONS
            .iter()
            .map(|s| s.to_string())
            .collect(),
    }
}

pub fn build_commit_message(commit_type: String, scope: String, subject: String) -> CommitMessage {
    CommitMessage::new(commit_type, scope, subject)
}

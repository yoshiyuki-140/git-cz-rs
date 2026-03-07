mod config;

use anyhow::Result;
use inquire::{Select, Text};
use std::process::Command;

// configモジュールの中から、公開されているものを使えるようにする
use config::{DEFAULT_PROMPT_OPTIONS, load_config};

fn main() -> Result<()> {
    // 設定を読み込む
    let config = load_config()?;

    // 読み込んだ設定が空でない場合、それを読み込むが、空の場合はデフォルトのハードコードされた値を使用する
    let prompt_options = if let Some(ref c) = config {
        c.options
            .iter()
            .map(|(key, desc)| format!("{key}: {desc}"))
            .collect::<Vec<String>>()
    } else {
        // cz.jsonが存在しない場合はデフォルトのハードコード設定を使う
        // ハードコードされた定数はVec<&str>型なのでto_stringを用いてString型に変換する
        DEFAULT_PROMPT_OPTIONS
            .iter()
            .map(|s| s.to_string())
            .collect()
    };

    let type_selection: String =
        Select::new("コミットタイプを選択してください", prompt_options).prompt()?;
    let commit_type = type_selection.split(':').next().unwrap();

    let scope_str = if let Some(Some(scopes)) = config.as_ref().map(|c| c.scopes.as_ref()) {
        let mut scope_options = vec!["(スキップ)".to_string()];
        scope_options.extend(scopes.clone());

        let selected_scope = Select::new("スコープを選択してください", scope_options).prompt()?;
        if selected_scope == "(スキップ)" {
            "".to_string()
        } else {
            format!("({selected_scope})")
        }
    } else {
        let scope =
            Text::new("スコープを入力してください(例: ui, parser) [Enterでスキップ]:").prompt()?;
        if scope.is_empty() {
            "".to_string()
        } else {
            format!("({scope})")
        }
    };

    // サマリーの入力
    let subject = Text::new("変更内容の要約を入力してください:").prompt()?;

    // メッセージの組み立て
    let commit_message = format!("{commit_type}{scope_str}: {subject}");

    println!("\n実行するコマンド: git commit -m \"{commit_message}\"");

    // gitコマンドの実行
    let status = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(&commit_message)
        .status()?;

    if status.success() {
        println!("コミットが完了しました！");
    } else {
        eprintln!("コミットに失敗しました。");
    }
    Ok(())
}

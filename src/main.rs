mod adapter;
mod driver;
mod entity;
mod usecase;

use anyhow::Result;

/// 設定を読み込み、対話形式でコミットメッセージを作成して git commit を実行する
fn main() -> Result<()> {
    let config = driver::load_config()?;
    let options = usecase::resolve_prompt_options(&config);
    let commit_type = adapter::select_commit_type(options)?;
    let scope = adapter::select_or_input_scope(&config)?;
    let subject = adapter::input_subject()?;
    let commit_message = usecase::build_commit_message(commit_type, scope, subject);
    let formatted = commit_message.format();

    println!("\n実行するコマンド: git commit -m \"{formatted}\"");

    if driver::execute_git_commit(&formatted)? {
        println!("コミットが完了しました！");
    } else {
        eprintln!("コミットに失敗しました。");
    }

    Ok(())
}

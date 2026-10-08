mod adapter;
mod driver;
mod entity;
mod usecase;

use anyhow::Result;

/// 各プログラムのオーケストレーションを行う
fn main() -> Result<()> {
    // `cz.json`の読み込み
    let config = driver::load_config()?;
    // ユーザと対話形式でコミット種別・スコープ・メッセージを取得
    let commit_type = adapter::select_commit_type(&config)?;
    let scope = adapter::select_or_input_scope(&config)?;
    let subject = adapter::input_subject(&config)?;
    // ユーザの出力を使って文字列に成形
    let commit_message = usecase::build_commit_message(commit_type, scope, subject);
    // gitコマンドの実行
    driver::execute_git_commit(&commit_message)?;
    Ok(())
}

# git-cz-rs

シンプルなcommitzen

## Install

- 前提
    - [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)をインストールして下さい

以下のコマンドを実行してください。

```bash
cargo install --git https://github.com/yoshiyuki-140/git-cz-rs.git
```

## Usage

installが完了したら`git cz-rs`でコミット時にプログラムが走るはずです。

<img width="2500" height="1000" alt="feature-3" src="https://github.com/user-attachments/assets/598767a1-f79c-44f9-bbe4-ecaada8b9496" />

## Option

`cz.json`を各プロジェクトに配置すればコマンド実行時の最も近い親ディレクトリの`cz.json`を参照します.
`cz.json`が空の場合は、ソースコードにハードコードされたデフォルトの設定が参照されます.

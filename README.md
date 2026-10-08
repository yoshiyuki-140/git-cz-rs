# git-cz-rs

シンプルなcommitzen

## Install

- 前提
    - cargoをインストールして下さい

以下のコマンドを実行してください。

```bash
cargo install --git https://github.com/yoshiyuki-140/git-cz-rs.git
```

## Usage

installが完了したら`git cz-rs`でコミット時にプログラムが走るはずです。

## Option

`cz.json`を各プロジェクトに配置すればコマンド実行時の最も近い親ディレクトリの`cz.json`を参照します.
`cz.json`が空の場合は、ソースコードにハードコードされたデフォルトの設定が参照されます.

# git-cz-rs

シンプルなcommitzen

## Install

- 前提
    - cargoをインストールして下さい

このリポジトリをクローン

```bash
git clone git@github.com:yoshiyuki-140/git-cz-rs.git /tmp
```

リポジトリに移動


```bash
cd /tmp/git-cz-rs
```

インストール

```bash
cargo install --path .
```

## Usage

installが完了したら`git cz-rs`でコミット時にプログラムが走るはずです。

## Option

`cz.json`を各プロジェクトに配置すればコマンド実行時の最も近い親ディレクトリの`cz.json`を参照します.
`cz.json`が空の場合は、ソースコードにハードコードされたデフォルトの設定が参照されます.
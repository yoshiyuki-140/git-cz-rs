# 開発者向け情報

このドキュメントは開発者向けの情報を記載します

## 動作確認方法

動作確認は簡単です。
開発者の環境にインストールしてしまえば良いです。

```bash
cd git-cz-rs # プロジェクトルートで実行すること
cargo install --path . # その時点でのソースコードでビルドしてインストール
```

## ファイルごとの責務

`./crates`ディレクトリ配下にライブラリクレートとして各層の責務が存在します。

```bash
$ tree crates/
crates/
├── adapter # ユーザとの対話
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── driver # gitコマンドの実行等, コンピュータとのI/O
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── entity # コミットメッセージの型や設定ファイル`cz.json`の型
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
└── usecase # アプリケーションが行う文字列結合の処理
    ├── Cargo.toml
    └── src
        └── lib.rs

9 directories, 8 files
```

これらのクレートを呼び出し、全体の処理を統合するコードが`./src`配下の`main.rs`になります。

## テスト

テストコードは単体テストのみとし、定義した処理と同一ファイル内に書いています。
driver, adapterはユーザやコンピュータとのI/Oの処理であるため、単体テストではなく動作確認で十分としています。


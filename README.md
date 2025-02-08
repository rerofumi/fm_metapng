# fm_metapng

日本語 | [English](README_en.md)

fm_metapng は、PNG ファイル内に埋め込まれた生成 AI のプロンプト情報を抽出し、表示するツールです。  
このリポジトリでは、実行時に PNG 内のテキストデータ（特に "parameters" キーワード）を解析し、プロンプト内容を表示します。

## 特長
- PNG ファイル内に埋め込まれたプロンプト情報の抽出
- "Negative prompt:" の部分は改行して見やすく表示
- プロンプト情報が存在しない場合には、その旨を出力

## 使用方法
1. PNG ファイルのパスを引数として指定して実行します。
    ```sh
    cargo run -- path/to/your_image.png
    ```
2. 正常な PNG ファイルが指定されると、プロンプト情報がコンソールに出力されます。

## ビルド済み実行ファイル
リリースからは、以下のプラットフォーム向けにビルド済みの実行ファイルをダウンロードできます:
- Windows
- Linux

## 依存関係
- [clap](https://crates.io/crates/clap) - コマンドライン引数のパース
- [png](https://crates.io/crates/png) - PNG ファイルの解析

## ライセンス
本プロジェクトは [MIT ライセンス](./LICENSE) の下で公開されています。

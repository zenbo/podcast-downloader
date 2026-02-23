# ADR-010: データ保存方式の簡素化

| 項目       | 値         |
| ---------- | ---------- |
| 日付       | 2026-02-22 |
| ステータス | 採用       |

## コンテキスト

当初の設計では SQLite に5テーブル（podcasts, episodes, download_history, settings, character_replacements）を使用する予定だった。設計レビューで各テーブルの必要性を再検討した。

## 変更内容

| テーブル               | 変更前              | 変更後                                      |
| ---------------------- | ------------------- | ------------------------------------------- |
| podcasts               | SQLite              | SQLite（変更なし）                          |
| episodes               | SQLite              | SQLite（`downloaded_at` カラム追加）        |
| download_history       | SQLite              | **廃止** — episodes.downloaded_at で代替    |
| settings               | SQLite（key-value） | **廃止** — tauri-plugin-store（JSON）へ移行 |
| character_replacements | SQLite              | **廃止** — tauri-plugin-store（JSON）へ移行 |

## 決定

- SQLite は **podcasts と episodes の2テーブルのみ** とする
- アプリケーション設定（download_dir, character_replacements, fallback_replacement）は **tauri-plugin-store** で JSON ファイルに保存する
- DL状態は episodes テーブルの `downloaded_at` カラム（NULL = 未DL）で管理する
- DL先ファイルパスは設定とエピソード情報から計算可能なため保存しない

## 理由

- **download_history 廃止**: DL成功のみ記録し、1エピソード1レコードの UNIQUE 制約を付ける設計になったため、episodes テーブルの1カラムで十分に表現できる。JOIN も不要になり新着判定 SQL が簡素化される
- **settings 廃止**: 保存するキーが `download_dir` の1件だけであり、汎用 key-value テーブルは過剰
- **character_replacements の JSON 移行**: 設定データでありリレーショナルな参照がない。JSON 配列で自然に表現でき、配列順序が適用順序になるため sort_order カラムも不要になる

## 影響・リスク

- ADR-003 の決定範囲が変わる（設定情報は SQLite ではなく JSON に保存）
- tauri-plugin-store への依存が追加される

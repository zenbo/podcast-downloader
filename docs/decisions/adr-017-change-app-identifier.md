# ADR-017: アプリ identifier の変更

## ステータス

採用

## コンテキスト

現在の Tauri アプリ identifier は `com.podcast-downloader.app` だが、以下の問題がある。

1. 逆ドメイン表記として `podcast-downloader.com` を所有していないため、他者と衝突する可能性がある
2. 個人ドメイン `zenbo.jp` も候補だが、将来手放す可能性があり永続性に欠ける

identifier は `tauri-plugin-store` の設定ファイルや SQLite データベースの保存先ディレクトリ名に使用されるため、変更するとパスが変わる。

## 判断

`com.podcast-downloader.app` → `com.github.zenbo.podcast-downloader` に変更する。

GitHub アカウント (`github.com/zenbo`) は失効リスクが低く、逆ドメイン表記としても一意性を担保できる。

## 影響範囲

- `src-tauri/tauri.conf.json` — `identifier` フィールドを変更

**注意:** identifier の変更により、OS 上の設定ファイル・DB の保存先ディレクトリ名が変わるため、既存データは引き継がれない。開発段階のため問題なしと判断した。

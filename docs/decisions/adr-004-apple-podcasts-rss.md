# ADR-004: Apple Podcasts URL からの RSS 取得方式

| 項目 | 値 |
|------|-----|
| 日付 | 2026-02-21 |
| ステータス | 採用 |

## コンテキスト

ユーザーが Apple Podcasts の Web ページ URL を入力した際に、対応する RSS フィード URL を取得する方式を決定する。

## 検討した選択肢

1. **iTunes Lookup API** — URL から Podcast ID を正規表現で抽出し、`https://itunes.apple.com/lookup?id={ID}&entity=podcast` で RSS フィード URL（`feedUrl` フィールド）を取得する
2. **HTML スクレイピング** — Apple Podcasts の Web ページの HTML を解析して RSS フィード URL を取得する

## 決定

**iTunes Lookup API** を使用する。

## 理由

- 公開 API であり、認証不要で利用できる
- JSON レスポンスから `feedUrl` フィールドを取得するだけでよく、実装がシンプル
- HTML スクレイピングに比べて、Apple 側のページ構造変更の影響を受けにくい
- デスクトップアプリ（Rust HTTP クライアント）からのアクセスのため、CORS の制約がない

## 影響・リスク

- Apple が API の仕様を変更した場合、対応が必要になる
- API で feedUrl が返されない番組が存在する可能性がある（その場合のエラーハンドリングが必要）

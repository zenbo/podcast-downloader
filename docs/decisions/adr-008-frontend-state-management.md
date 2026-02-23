# ADR-008: フロントエンド状態管理の選定

| 項目       | 値         |
| ---------- | ---------- |
| 日付       | 2026-02-22 |
| ステータス | 採用       |

## コンテキスト

ADR-002 で React + TypeScript を採用した際、状態管理ライブラリの選定が別途必要と記録されていた。サーバー状態（番組・エピソード等の DB データ）と UI 状態（ダイアログ表示、DL進捗等）の管理方法を決定する。

## 検討した選択肢

1. **TanStack Query + useState** — サーバー状態は TanStack Query、UI 状態は useState + props で管理
2. **TanStack Query + Zustand** — グローバル UI 状態（DL進捗等）を Zustand で管理
3. **TanStack Query + useState + 必要時に Zustand 追加** — useState で始めて、必要になったら Zustand を導入

## 決定

**選択肢 1: TanStack Query + useState** を採用する。Zustand は導入しない。

## 理由

- 画面が3つしかなく、コンポーネント階層が浅い
- DL進捗はステータスバーに表示するが、DLを開始するページと同じ画面内にあるため、共通の親コンポーネントから props で渡せる
- 個人利用アプリであり、最小限の依存で十分

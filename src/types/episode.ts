/** データベースから取得した完全なエピソード情報 */
export interface Episode {
  id: number;
  podcastId: number;
  guid: string;
  title: string;
  description: string | null;
  audioUrl: string;
  fileSize: number | null;
  publishedAt: string;
  downloadedAt: string | null;
  createdAt: string;
  /** バックエンドの新着判定ロジックに基づく新着フラグ */
  isNew: boolean;
}

/** 新着チェック結果（単一番組） */
export interface CheckNewResult {
  /** 現在の新着エピソード数（既存 + 今回発見） */
  newCount: number;
  /** 今回のチェックで新たに見つかったエピソード数 */
  newlyFoundCount: number;
}

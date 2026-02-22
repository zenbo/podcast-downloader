/** データベースから取得した完全な番組情報 */
export interface Podcast {
  id: number;
  title: string;
  author: string | null;
  description: string | null;
  feedUrl: string;
  applePodcastsUrl: string | null;
  imageUrl: string | null;
  lastCheckedAt: string | null;
  createdAt: string;
  updatedAt: string;
}

/** 番組一覧表示用（新着エピソード数を含む） */
export interface PodcastSummary {
  id: number;
  title: string;
  author: string | null;
  imageUrl: string | null;
  newEpisodeCount: number;
}

/** 全番組の新着チェック結果 */
export interface PodcastNewCount {
  podcastId: number;
  title: string;
  newCount: number;
}

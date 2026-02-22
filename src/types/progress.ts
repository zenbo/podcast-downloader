/** 個別エピソードのダウンロード進捗 */
export interface DownloadProgress {
  episodeId: number;
  downloadedBytes: number;
  totalBytes: number | null;
  percentage: number | null;
}

/** 一括ダウンロードの進捗 */
export interface BatchDownloadProgress {
  currentEpisodeId: number;
  currentEpisodeTitle: string;
  episodeProgress: DownloadProgress;
  completedCount: number;
  totalCount: number;
}

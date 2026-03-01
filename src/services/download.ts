import { invoke, Channel } from "@tauri-apps/api/core";
import type { DownloadProgress, BatchDownloadProgress, BatchDownloadSummary } from "@/types";

/** エピソードをダウンロードする（進捗コールバック付き） */
export async function downloadEpisode(
  episodeId: number,
  onProgress: (progress: DownloadProgress) => void,
): Promise<void> {
  const channel = new Channel<DownloadProgress>();
  channel.onmessage = onProgress;
  return invoke<void>("download_episode", {
    episodeId,
    onProgress: channel,
  });
}

/** 選択番組の新着エピソードを一括ダウンロードする */
export async function batchDownloadNew(
  podcastIds: number[],
  onProgress: (progress: BatchDownloadProgress) => void,
): Promise<BatchDownloadSummary> {
  const channel = new Channel<BatchDownloadProgress>();
  channel.onmessage = onProgress;
  return invoke<BatchDownloadSummary>("batch_download_new", {
    podcastIds,
    onProgress: channel,
  });
}

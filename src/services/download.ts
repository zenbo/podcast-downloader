import { invoke, Channel } from "@tauri-apps/api/core";
import type { DownloadProgress, BatchDownloadProgress, BatchDownloadSummary } from "@/types";

/** エピソードをDLせずにDL済み扱いにする（スキップ） */
export async function skipEpisode(episodeId: number): Promise<void> {
  return invoke<void>("skip_episode", { episodeId });
}

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

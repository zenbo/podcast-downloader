import { invoke } from "@tauri-apps/api/core";
import type { Podcast, PodcastSummary } from "@/types";

/** Apple Podcasts URL または RSS フィード URL から番組を登録する */
export async function registerPodcast(url: string): Promise<Podcast> {
  return invoke<Podcast>("register_podcast", { url });
}

/** 番組一覧を取得する（新着エピソード数付き） */
export async function listPodcasts(): Promise<PodcastSummary[]> {
  return invoke<PodcastSummary[]>("list_podcasts");
}

/** 番組を削除する */
export async function deletePodcast(podcastId: number): Promise<void> {
  return invoke<void>("delete_podcast", { podcastId });
}

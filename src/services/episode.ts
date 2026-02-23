import { invoke } from "@tauri-apps/api/core";
import type { Episode, CheckNewResult, PodcastNewCount } from "@/types";

/** 番組のエピソード一覧を取得する（配信日降順） */
export async function listEpisodes(podcastId: number): Promise<Episode[]> {
  return invoke<Episode[]>("list_episodes", { podcastId });
}

/** 番組の新着エピソードをチェックする */
export async function checkNewEpisodes(podcastId: number): Promise<CheckNewResult> {
  return invoke<CheckNewResult>("check_new_episodes", { podcastId });
}

/** 全番組の新着をチェックする */
export async function checkAllNew(): Promise<PodcastNewCount[]> {
  return invoke<PodcastNewCount[]>("check_all_new");
}

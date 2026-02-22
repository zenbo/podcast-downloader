import { invoke } from "@tauri-apps/api/core";
import type { AppSettings } from "@/types";

/** アプリ設定を取得する */
export async function getSettings(): Promise<AppSettings> {
  return invoke<AppSettings>("get_settings");
}

/** アプリ設定を保存する */
export async function updateSettings(settings: AppSettings): Promise<void> {
  return invoke<void>("update_settings", { settings });
}

/** フォルダ選択ダイアログを表示する */
export async function selectFolder(): Promise<string | null> {
  return invoke<string | null>("select_folder");
}

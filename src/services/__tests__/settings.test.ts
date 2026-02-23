import { describe, it, expect, vi, beforeEach } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import { getSettings, updateSettings, selectFolder } from "@/services/settings";
import type { AppSettings } from "@/types";

const mockInvoke = vi.mocked(invoke);

describe("settings service", () => {
  beforeEach(() => {
    mockInvoke.mockReset();
  });

  describe("getSettings", () => {
    it("invoke を 'get_settings' コマンドで呼び出す", async () => {
      const mockSettings: AppSettings = {
        downloadDir: "/Users/test/Downloads",
        characterReplacements: [{ before: "?", after: "_" }],
        fallbackReplacement: "_",
      };
      mockInvoke.mockResolvedValue(mockSettings);

      const result = await getSettings();

      expect(mockInvoke).toHaveBeenCalledWith("get_settings");
      expect(result).toEqual(mockSettings);
    });
  });

  describe("updateSettings", () => {
    it("invoke を 'update_settings' コマンドと settings 引数で呼び出す", async () => {
      const settings: AppSettings = {
        downloadDir: "/new/path",
        characterReplacements: [],
        fallbackReplacement: "-",
      };
      mockInvoke.mockResolvedValue(undefined);

      await updateSettings(settings);

      expect(mockInvoke).toHaveBeenCalledWith("update_settings", { settings });
    });
  });

  describe("selectFolder", () => {
    it("invoke を 'select_folder' コマンドで呼び出し、選択結果を返す", async () => {
      mockInvoke.mockResolvedValue("/selected/folder");

      const result = await selectFolder();

      expect(mockInvoke).toHaveBeenCalledWith("select_folder");
      expect(result).toBe("/selected/folder");
    });

    it("キャンセル時に null を返す", async () => {
      mockInvoke.mockResolvedValue(null);

      const result = await selectFolder();

      expect(result).toBeNull();
    });
  });
});

import { describe, it, expect, vi, beforeEach } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import {
  listEpisodes,
  checkNewEpisodes,
  checkAllNew,
} from "@/services/episode";
import type { Episode, PodcastNewCount } from "@/types";

const mockInvoke = vi.mocked(invoke);

describe("episode service", () => {
  beforeEach(() => {
    mockInvoke.mockReset();
  });

  describe("listEpisodes", () => {
    it("invoke を 'list_episodes' コマンドと podcastId 引数で呼び出す", async () => {
      const mockEpisodes: Episode[] = [
        {
          id: 1,
          podcastId: 10,
          guid: "guid-1",
          title: "Episode 1",
          description: null,
          audioUrl: "https://example.com/ep1.mp3",
          fileSize: 12345678,
          publishedAt: "2025-01-01T00:00:00Z",
          downloadedAt: null,
          createdAt: "2025-01-01T00:00:00Z",
          isNew: false,
        },
      ];
      mockInvoke.mockResolvedValue(mockEpisodes);

      const result = await listEpisodes(10);

      expect(mockInvoke).toHaveBeenCalledWith("list_episodes", {
        podcastId: 10,
      });
      expect(result).toEqual(mockEpisodes);
    });
  });

  describe("checkNewEpisodes", () => {
    it("invoke を 'check_new_episodes' コマンドと podcastId 引数で呼び出す", async () => {
      const mockResult = { newCount: 2, newlyFoundCount: 2 };
      mockInvoke.mockResolvedValue(mockResult);

      const result = await checkNewEpisodes(5);

      expect(mockInvoke).toHaveBeenCalledWith("check_new_episodes", {
        podcastId: 5,
      });
      expect(result).toEqual(mockResult);
    });
  });

  describe("checkAllNew", () => {
    it("invoke を 'check_all_new' コマンドで呼び出す", async () => {
      const mockCounts: PodcastNewCount[] = [
        { podcastId: 1, title: "Podcast A", newCount: 3, newlyFoundCount: 3 },
      ];
      mockInvoke.mockResolvedValue(mockCounts);

      const result = await checkAllNew();

      expect(mockInvoke).toHaveBeenCalledWith("check_all_new");
      expect(result).toEqual(mockCounts);
    });
  });
});

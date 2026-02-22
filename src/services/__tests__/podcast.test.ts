import { describe, it, expect, vi, beforeEach } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import { registerPodcast, listPodcasts, deletePodcast } from "@/services/podcast";
import type { Podcast, PodcastSummary } from "@/types";

const mockInvoke = vi.mocked(invoke);

describe("podcast service", () => {
  beforeEach(() => {
    mockInvoke.mockReset();
  });

  describe("registerPodcast", () => {
    it("invoke を 'register_podcast' コマンドと url 引数で呼び出す", async () => {
      const mockPodcast: Podcast = {
        id: 1,
        title: "Test Podcast",
        author: "Author",
        description: "Description",
        feedUrl: "https://example.com/feed.xml",
        applePodcastsUrl: null,
        imageUrl: null,
        lastCheckedAt: null,
        createdAt: "2025-01-01T00:00:00Z",
        updatedAt: "2025-01-01T00:00:00Z",
      };
      mockInvoke.mockResolvedValue(mockPodcast);

      const result = await registerPodcast(
        "https://podcasts.apple.com/podcast/id12345",
      );

      expect(mockInvoke).toHaveBeenCalledWith("register_podcast", {
        url: "https://podcasts.apple.com/podcast/id12345",
      });
      expect(result).toEqual(mockPodcast);
    });
  });

  describe("listPodcasts", () => {
    it("invoke を 'list_podcasts' コマンドで呼び出し、結果をそのまま返す", async () => {
      const mockSummaries: PodcastSummary[] = [
        {
          id: 1,
          title: "Podcast A",
          author: null,
          imageUrl: null,
          newEpisodeCount: 3,
        },
      ];
      mockInvoke.mockResolvedValue(mockSummaries);

      const result = await listPodcasts();

      expect(mockInvoke).toHaveBeenCalledWith("list_podcasts");
      expect(result).toEqual(mockSummaries);
    });
  });

  describe("deletePodcast", () => {
    it("invoke を 'delete_podcast' コマンドと podcastId 引数で呼び出す", async () => {
      mockInvoke.mockResolvedValue(undefined);

      await deletePodcast(42);

      expect(mockInvoke).toHaveBeenCalledWith("delete_podcast", {
        podcastId: 42,
      });
    });
  });
});

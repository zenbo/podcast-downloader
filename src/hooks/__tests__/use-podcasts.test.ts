import { describe, it, expect, vi, beforeEach } from "vitest";
import { waitFor } from "@testing-library/react";
import { renderHookWithQueryClient } from "@/test/test-utils";
import {
  usePodcasts,
  useRegisterPodcast,
  useDeletePodcast,
  useCheckAllNew,
} from "@/hooks/use-podcasts";
import type { Podcast, PodcastSummary, PodcastNewCount } from "@/types";

vi.mock("@/services/podcast", () => ({
  listPodcasts: vi.fn(),
  registerPodcast: vi.fn(),
  deletePodcast: vi.fn(),
}));

vi.mock("@/services/episode", () => ({
  checkAllNew: vi.fn(),
}));

import { listPodcasts, registerPodcast, deletePodcast } from "@/services/podcast";
import { checkAllNew } from "@/services/episode";

const mockListPodcasts = vi.mocked(listPodcasts);
const mockRegisterPodcast = vi.mocked(registerPodcast);
const mockDeletePodcast = vi.mocked(deletePodcast);
const mockCheckAllNew = vi.mocked(checkAllNew);

describe("use-podcasts hooks", () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  describe("usePodcasts", () => {
    it("listPodcasts を呼び出してデータを返す", async () => {
      const mockData: PodcastSummary[] = [
        {
          id: 1,
          title: "Test",
          author: null,
          imageUrl: null,
          newEpisodeCount: 0,
        },
      ];
      mockListPodcasts.mockResolvedValue(mockData);

      const { result } = renderHookWithQueryClient(() => usePodcasts());

      await waitFor(() => expect(result.current.isSuccess).toBe(true));
      expect(result.current.data).toEqual(mockData);
      expect(mockListPodcasts).toHaveBeenCalledOnce();
    });
  });

  describe("useRegisterPodcast", () => {
    it("registerPodcast を呼び出し、成功時に podcasts クエリを無効化する", async () => {
      const mockPodcast: Podcast = {
        id: 1,
        title: "New Podcast",
        author: null,
        description: null,
        feedUrl: "https://example.com/feed.xml",
        applePodcastsUrl: null,
        imageUrl: null,
        lastCheckedAt: null,
        createdAt: "2025-01-01T00:00:00Z",
        updatedAt: "2025-01-01T00:00:00Z",
      };
      mockRegisterPodcast.mockResolvedValue(mockPodcast);

      const { result, queryClient } = renderHookWithQueryClient(() => useRegisterPodcast());
      const invalidateSpy = vi.spyOn(queryClient, "invalidateQueries");

      result.current.mutate("https://podcasts.apple.com/podcast/id12345");

      await waitFor(() => expect(result.current.isSuccess).toBe(true));
      expect(mockRegisterPodcast).toHaveBeenCalledWith(
        "https://podcasts.apple.com/podcast/id12345",
      );
      expect(invalidateSpy).toHaveBeenCalledWith({
        queryKey: ["podcasts"],
      });
    });
  });

  describe("useDeletePodcast", () => {
    it("deletePodcast を呼び出し、成功時に podcasts クエリを無効化する", async () => {
      mockDeletePodcast.mockResolvedValue(undefined);

      const { result, queryClient } = renderHookWithQueryClient(() => useDeletePodcast());
      const invalidateSpy = vi.spyOn(queryClient, "invalidateQueries");

      result.current.mutate(42);

      await waitFor(() => expect(result.current.isSuccess).toBe(true));
      expect(mockDeletePodcast).toHaveBeenCalledWith(42);
      expect(invalidateSpy).toHaveBeenCalledWith({
        queryKey: ["podcasts"],
      });
    });
  });

  describe("useCheckAllNew", () => {
    it("checkAllNew を呼び出し、成功時に podcasts クエリを無効化する", async () => {
      const mockCounts: PodcastNewCount[] = [
        { podcastId: 1, title: "A", newCount: 2, newlyFoundCount: 2 },
      ];
      mockCheckAllNew.mockResolvedValue(mockCounts);

      const { result, queryClient } = renderHookWithQueryClient(() => useCheckAllNew());
      const invalidateSpy = vi.spyOn(queryClient, "invalidateQueries");

      result.current.mutate();

      await waitFor(() => expect(result.current.isSuccess).toBe(true));
      expect(mockCheckAllNew).toHaveBeenCalledOnce();
      expect(invalidateSpy).toHaveBeenCalledWith({
        queryKey: ["podcasts"],
      });
    });
  });
});

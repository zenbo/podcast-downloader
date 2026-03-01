import { describe, it, expect, vi, beforeEach } from "vitest";
import { waitFor } from "@testing-library/react";
import { renderHookWithQueryClient } from "@/test/test-utils";
import { useEpisodes, useCheckNewEpisodes, useSkipEpisode } from "@/hooks/use-episodes";
import type { Episode } from "@/types";

vi.mock("@/services/episode", () => ({
  listEpisodes: vi.fn(),
  checkNewEpisodes: vi.fn(),
}));

vi.mock("@/services/download", () => ({
  skipEpisode: vi.fn(),
}));

import { listEpisodes, checkNewEpisodes } from "@/services/episode";
import { skipEpisode } from "@/services/download";

const mockListEpisodes = vi.mocked(listEpisodes);
const mockCheckNewEpisodes = vi.mocked(checkNewEpisodes);
const mockSkipEpisode = vi.mocked(skipEpisode);

describe("use-episodes hooks", () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  describe("useEpisodes", () => {
    it("listEpisodes を podcastId を引数にして呼び出す", async () => {
      const mockEpisodes: Episode[] = [
        {
          id: 1,
          podcastId: 10,
          guid: "guid-1",
          title: "Ep 1",
          description: null,
          audioUrl: "https://example.com/ep1.mp3",
          fileSize: null,
          publishedAt: "2025-01-01T00:00:00Z",
          downloadedAt: null,
          createdAt: "2025-01-01T00:00:00Z",
          isNew: true,
        },
      ];
      mockListEpisodes.mockResolvedValue(mockEpisodes);

      const { result } = renderHookWithQueryClient(() => useEpisodes(10));

      await waitFor(() => expect(result.current.isSuccess).toBe(true));
      expect(result.current.data).toEqual(mockEpisodes);
      expect(mockListEpisodes).toHaveBeenCalledWith(10);
    });
  });

  describe("useSkipEpisode", () => {
    it("skipEpisode を呼び出し、成功時に episodes と podcasts クエリを無効化する", async () => {
      mockSkipEpisode.mockResolvedValue(undefined);

      const { result, queryClient } = renderHookWithQueryClient(() => useSkipEpisode(10));
      const invalidateSpy = vi.spyOn(queryClient, "invalidateQueries");

      result.current.mutate(42);

      await waitFor(() => expect(result.current.isSuccess).toBe(true));
      expect(mockSkipEpisode).toHaveBeenCalledWith(42);
      expect(invalidateSpy).toHaveBeenCalledWith({
        queryKey: ["episodes", "list", 10],
      });
      expect(invalidateSpy).toHaveBeenCalledWith({
        queryKey: ["podcasts"],
      });
    });
  });

  describe("useCheckNewEpisodes", () => {
    it("checkNewEpisodes を呼び出し、成功時に episodes と podcasts クエリを無効化する", async () => {
      mockCheckNewEpisodes.mockResolvedValue({ newCount: 0, newlyFoundCount: 0 });

      const { result, queryClient } = renderHookWithQueryClient(() => useCheckNewEpisodes());
      const invalidateSpy = vi.spyOn(queryClient, "invalidateQueries");

      result.current.mutate(10);

      await waitFor(() => expect(result.current.isSuccess).toBe(true));
      expect(mockCheckNewEpisodes).toHaveBeenCalledWith(10);
      expect(invalidateSpy).toHaveBeenCalledWith({
        queryKey: ["episodes", "list", 10],
      });
      expect(invalidateSpy).toHaveBeenCalledWith({
        queryKey: ["podcasts"],
      });
    });
  });
});

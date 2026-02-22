import { describe, it, expect, vi, beforeEach } from "vitest";
import { invoke, Channel } from "@tauri-apps/api/core";
import { downloadEpisode, batchDownloadNew } from "@/services/download";

const mockInvoke = vi.mocked(invoke);

describe("download service", () => {
  beforeEach(() => {
    mockInvoke.mockReset();
  });

  describe("downloadEpisode", () => {
    it("Channel を作成し、onmessage にコールバックを設定して invoke に渡す", async () => {
      mockInvoke.mockResolvedValue(undefined);
      const onProgress = vi.fn();

      await downloadEpisode(42, onProgress);

      expect(mockInvoke).toHaveBeenCalledWith("download_episode", {
        episodeId: 42,
        onProgress: expect.any(Channel),
      });

      const args = mockInvoke.mock.calls[0][1] as Record<string, unknown>;
      const passedChannel = args.onProgress as InstanceType<typeof Channel>;
      expect(passedChannel.onmessage).toBe(onProgress);
    });
  });

  describe("batchDownloadNew", () => {
    it("Channel を作成し、onmessage にコールバックを設定して invoke に渡す", async () => {
      mockInvoke.mockResolvedValue(undefined);
      const onProgress = vi.fn();

      await batchDownloadNew([1, 2, 3], onProgress);

      expect(mockInvoke).toHaveBeenCalledWith("batch_download_new", {
        podcastIds: [1, 2, 3],
        onProgress: expect.any(Channel),
      });

      const args = mockInvoke.mock.calls[0][1] as Record<string, unknown>;
      const passedChannel = args.onProgress as InstanceType<typeof Channel>;
      expect(passedChannel.onmessage).toBe(onProgress);
    });
  });
});

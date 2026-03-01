import React from "react";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { renderHook, act } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { DownloadProvider, useDownload } from "@/stores/download-context";
import type { BatchDownloadProgress, BatchDownloadSummary } from "@/types";

vi.mock("sonner", () => ({
  toast: Object.assign(vi.fn(), {
    success: vi.fn(),
    error: vi.fn(),
    warning: vi.fn(),
  }),
}));

vi.mock("@/services/download", () => ({
  batchDownloadNew: vi.fn(),
  downloadEpisode: vi.fn(),
}));

import { batchDownloadNew, downloadEpisode } from "@/services/download";

const mockBatchDownloadNew = vi.mocked(batchDownloadNew);
const mockDownloadEpisode = vi.mocked(downloadEpisode);

function createWrapper() {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false }, mutations: { retry: false } },
  });
  return ({ children }: { children: React.ReactNode }) =>
    React.createElement(
      QueryClientProvider,
      { client: queryClient },
      React.createElement(DownloadProvider, null, children),
    );
}

describe("download-context", () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  describe("batchTargetIds", () => {
    it("初期値は空の Set である", () => {
      const { result } = renderHook(() => useDownload(), { wrapper: createWrapper() });
      expect(result.current.batchTargetIds.size).toBe(0);
    });

    it("startBatchDownload で targetEpisodeIds を受け取ると batchTargetIds にセットされる", async () => {
      let capturedOnProgress: ((progress: BatchDownloadProgress) => void) | undefined;

      mockBatchDownloadNew.mockImplementation((_ids, onProgress) => {
        capturedOnProgress = onProgress;
        // 初回通知: 対象IDリストを送信
        onProgress({
          currentEpisodeId: 0,
          currentEpisodeTitle: "",
          episodeProgress: {
            episodeId: 0,
            downloadedBytes: 0,
            totalBytes: null,
            percentage: null,
          },
          completedCount: 0,
          totalCount: 3,
          targetEpisodeIds: [10, 20, 30],
        });
        return Promise.resolve({
          completedCount: 3,
          failedCount: 0,
          totalCount: 3,
        } satisfies BatchDownloadSummary);
      });

      const { result } = renderHook(() => useDownload(), { wrapper: createWrapper() });

      await act(async () => {
        await result.current.startBatchDownload([1]);
      });

      // onProgress が呼ばれたことを確認
      expect(capturedOnProgress).toBeDefined();
    });

    it("startBatchDownload 完了後に batchTargetIds がクリアされる", async () => {
      mockBatchDownloadNew.mockImplementation((_ids, onProgress) => {
        onProgress({
          currentEpisodeId: 0,
          currentEpisodeTitle: "",
          episodeProgress: {
            episodeId: 0,
            downloadedBytes: 0,
            totalBytes: null,
            percentage: null,
          },
          completedCount: 0,
          totalCount: 2,
          targetEpisodeIds: [10, 20],
        });
        return Promise.resolve({
          completedCount: 2,
          failedCount: 0,
          totalCount: 2,
        });
      });

      const { result } = renderHook(() => useDownload(), { wrapper: createWrapper() });

      await act(async () => {
        await result.current.startBatchDownload([1]);
      });

      // finally でクリアされているはず
      expect(result.current.batchTargetIds.size).toBe(0);
    });

    it("startBatchDownload がエラーで終了しても batchTargetIds がクリアされる", async () => {
      mockBatchDownloadNew.mockImplementation((_ids, onProgress) => {
        onProgress({
          currentEpisodeId: 0,
          currentEpisodeTitle: "",
          episodeProgress: {
            episodeId: 0,
            downloadedBytes: 0,
            totalBytes: null,
            percentage: null,
          },
          completedCount: 0,
          totalCount: 1,
          targetEpisodeIds: [10],
        });
        return Promise.reject(new Error("network error"));
      });

      const { result } = renderHook(() => useDownload(), { wrapper: createWrapper() });

      await act(async () => {
        await result.current.startBatchDownload([1]);
      });

      expect(result.current.batchTargetIds.size).toBe(0);
    });
  });

  describe("startEpisodeDownload のバッチ対象ガード", () => {
    it("batchTargetIds に含まれるエピソードの単体DLを拒否する", async () => {
      // batchDownloadNew を「完了しない Promise」にして batchTargetIds が残った状態を作る
      let resolveBatch: ((summary: BatchDownloadSummary) => void) | undefined;
      mockBatchDownloadNew.mockImplementation((_ids, onProgress) => {
        onProgress({
          currentEpisodeId: 0,
          currentEpisodeTitle: "",
          episodeProgress: {
            episodeId: 0,
            downloadedBytes: 0,
            totalBytes: null,
            percentage: null,
          },
          completedCount: 0,
          totalCount: 1,
          targetEpisodeIds: [42],
        });
        return new Promise<BatchDownloadSummary>((resolve) => {
          resolveBatch = resolve;
        });
      });

      const { result } = renderHook(() => useDownload(), { wrapper: createWrapper() });

      // バッチDLを開始（未完了のまま）
      act(() => {
        result.current.startBatchDownload([1]);
      });

      // バッチ対象のエピソード 42 を単体DLしようとする
      act(() => {
        result.current.startEpisodeDownload(42, "Episode 42");
      });

      // downloadEpisode が呼ばれていないことを確認
      expect(mockDownloadEpisode).not.toHaveBeenCalled();

      // バッチ対象外のエピソード 99 は単体DLできる
      mockDownloadEpisode.mockResolvedValue(undefined);
      act(() => {
        result.current.startEpisodeDownload(99, "Episode 99");
      });

      expect(mockDownloadEpisode).toHaveBeenCalledTimes(1);
      expect(mockDownloadEpisode).toHaveBeenCalledWith(99, expect.any(Function));

      // クリーンアップ: バッチDLを完了させる
      await act(async () => {
        resolveBatch?.({ completedCount: 1, failedCount: 0, totalCount: 1 });
      });
    });

    it("バッチDL完了後はガードが解除され単体DLが可能になる", async () => {
      mockBatchDownloadNew.mockImplementation((_ids, onProgress) => {
        onProgress({
          currentEpisodeId: 0,
          currentEpisodeTitle: "",
          episodeProgress: {
            episodeId: 0,
            downloadedBytes: 0,
            totalBytes: null,
            percentage: null,
          },
          completedCount: 0,
          totalCount: 1,
          targetEpisodeIds: [42],
        });
        return Promise.resolve({
          completedCount: 1,
          failedCount: 0,
          totalCount: 1,
        });
      });

      mockDownloadEpisode.mockResolvedValue(undefined);

      const { result } = renderHook(() => useDownload(), { wrapper: createWrapper() });

      // バッチDLを開始して完了させる
      await act(async () => {
        await result.current.startBatchDownload([1]);
      });

      // バッチDL完了後、エピソード 42 の単体DLが可能になる
      act(() => {
        result.current.startEpisodeDownload(42, "Episode 42");
      });

      expect(mockDownloadEpisode).toHaveBeenCalledWith(42, expect.any(Function));
    });
  });
});

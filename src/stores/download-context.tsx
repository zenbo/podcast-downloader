import { createContext, useCallback, useContext, useMemo, useRef, useState } from "react";
import type { ReactNode } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { toast } from "sonner";

import type { BatchDownloadProgress, BatchDownloadSummary, DownloadProgress } from "@/types";
import { batchDownloadNew, downloadEpisode } from "@/services/download";
import { podcastKeys } from "@/hooks/use-podcasts";
import { episodeKeys } from "@/hooks/use-episodes";

interface DownloadContextValue {
  batchProgress: BatchDownloadProgress | null;
  isBatchDownloading: boolean;
  downloadingIds: ReadonlySet<number>;
  batchTargetIds: ReadonlySet<number>;
  progressMap: ReadonlyMap<number, { progress: DownloadProgress; title: string }>;
  startBatchDownload: (podcastIds: number[]) => Promise<BatchDownloadSummary | undefined>;
  startEpisodeDownload: (episodeId: number, episodeTitle: string) => void;
}

const DownloadContext = createContext<DownloadContextValue | null>(null);

export function useDownload(): DownloadContextValue {
  const ctx = useContext(DownloadContext);
  if (!ctx) throw new Error("useDownload must be used within DownloadProvider");
  return ctx;
}

export function DownloadProvider({ children }: { children: ReactNode }) {
  const queryClient = useQueryClient();

  const [batchProgress, setBatchProgress] = useState<BatchDownloadProgress | null>(null);
  const [isBatchDownloading, setIsBatchDownloading] = useState(false);
  const [downloadingIds, setDownloadingIds] = useState<Set<number>>(new Set());
  const [progressMap, setProgressMap] = useState<
    Map<number, { progress: DownloadProgress; title: string }>
  >(new Map());

  const [batchTargetIds, setBatchTargetIds] = useState<Set<number>>(new Set());

  const isBatchDownloadingRef = useRef(false);
  const downloadingIdsRef = useRef<Set<number>>(new Set());
  const batchTargetIdsRef = useRef<Set<number>>(new Set());
  const prevBatchEpisodeIdRef = useRef<number>(0);

  const startBatchDownload = useCallback(
    (podcastIds: number[]): Promise<BatchDownloadSummary | undefined> => {
      if (isBatchDownloadingRef.current) return Promise.resolve(undefined);
      isBatchDownloadingRef.current = true;
      setIsBatchDownloading(true);

      prevBatchEpisodeIdRef.current = 0;

      return batchDownloadNew(podcastIds, (progress) => {
        if (progress.targetEpisodeIds) {
          const ids = new Set(progress.targetEpisodeIds);
          batchTargetIdsRef.current = ids;
          setBatchTargetIds(ids);
        }

        // currentEpisodeId が切り替わった場合、前のエピソードが完了したため
        // エピソードクエリを再取得して downloadedAt を反映する
        if (
          progress.currentEpisodeId !== 0 &&
          progress.currentEpisodeId !== prevBatchEpisodeIdRef.current
        ) {
          if (prevBatchEpisodeIdRef.current !== 0) {
            queryClient.invalidateQueries({ queryKey: episodeKeys.all });
          }
          prevBatchEpisodeIdRef.current = progress.currentEpisodeId;
        }

        setBatchProgress(progress);
      })
        .then((summary: BatchDownloadSummary) => {
          if (summary.failedCount === 0) {
            toast.success("ダウンロードが完了しました");
          } else if (summary.completedCount === 0) {
            toast.error(`全 ${summary.totalCount} 件のダウンロードに失敗しました`);
          } else {
            toast.warning(
              `${summary.totalCount} 件中 ${summary.completedCount} 件成功、${summary.failedCount} 件失敗`,
            );
          }
          queryClient.invalidateQueries({ queryKey: podcastKeys.all });
          queryClient.invalidateQueries({ queryKey: episodeKeys.all });
          return summary;
        })
        .catch((err) => {
          toast.error(String(err));
          return undefined;
        })
        .finally(() => {
          setBatchProgress(null);
          setIsBatchDownloading(false);
          isBatchDownloadingRef.current = false;
          batchTargetIdsRef.current = new Set();
          setBatchTargetIds(new Set());
        });
    },
    [queryClient],
  );

  const startEpisodeDownload = useCallback(
    (episodeId: number, episodeTitle: string) => {
      if (downloadingIdsRef.current.has(episodeId)) return;
      if (batchTargetIdsRef.current.has(episodeId)) return;
      downloadingIdsRef.current = new Set(downloadingIdsRef.current).add(episodeId);
      setDownloadingIds((prev) => new Set(prev).add(episodeId));

      downloadEpisode(episodeId, (progress) => {
        setProgressMap((prev) => new Map(prev).set(episodeId, { progress, title: episodeTitle }));
      })
        .then(() => {
          toast.success("ダウンロードが完了しました");
          queryClient.invalidateQueries({ queryKey: episodeKeys.all });
        })
        .catch((err) => {
          toast.error(String(err));
        })
        .finally(() => {
          downloadingIdsRef.current = new Set(downloadingIdsRef.current);
          downloadingIdsRef.current.delete(episodeId);
          setDownloadingIds((prev) => {
            const next = new Set(prev);
            next.delete(episodeId);
            return next;
          });
          setProgressMap((prev) => {
            const next = new Map(prev);
            next.delete(episodeId);
            return next;
          });
        });
    },
    [queryClient],
  );

  const value = useMemo(
    () => ({
      batchProgress,
      isBatchDownloading,
      downloadingIds,
      batchTargetIds,
      progressMap,
      startBatchDownload,
      startEpisodeDownload,
    }),
    [
      batchProgress,
      isBatchDownloading,
      downloadingIds,
      batchTargetIds,
      progressMap,
      startBatchDownload,
      startEpisodeDownload,
    ],
  );

  return <DownloadContext.Provider value={value}>{children}</DownloadContext.Provider>;
}

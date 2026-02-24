import { createContext, useCallback, useContext, useMemo, useRef, useState } from "react";
import type { ReactNode } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { toast } from "sonner";

import type { BatchDownloadProgress, DownloadProgress } from "@/types";
import { batchDownloadNew, downloadEpisode } from "@/services/download";
import { podcastKeys } from "@/hooks/use-podcasts";
import { episodeKeys } from "@/hooks/use-episodes";

interface DownloadContextValue {
  batchProgress: BatchDownloadProgress | null;
  isBatchDownloading: boolean;
  downloadingIds: ReadonlySet<number>;
  progressMap: ReadonlyMap<number, { progress: DownloadProgress; title: string }>;
  startBatchDownload: (podcastIds: number[]) => void;
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

  const isBatchDownloadingRef = useRef(false);
  const downloadingIdsRef = useRef<Set<number>>(new Set());

  const startBatchDownload = useCallback(
    (podcastIds: number[]) => {
      if (isBatchDownloadingRef.current) return;
      isBatchDownloadingRef.current = true;
      setIsBatchDownloading(true);

      batchDownloadNew(podcastIds, (progress) => {
        setBatchProgress(progress);
      })
        .then(() => {
          toast.success("ダウンロードが完了しました");
          queryClient.invalidateQueries({ queryKey: podcastKeys.all });
          queryClient.invalidateQueries({ queryKey: episodeKeys.all });
        })
        .catch((err) => {
          toast.error(String(err));
        })
        .finally(() => {
          setBatchProgress(null);
          setIsBatchDownloading(false);
          isBatchDownloadingRef.current = false;
        });
    },
    [queryClient],
  );

  const startEpisodeDownload = useCallback(
    (episodeId: number, episodeTitle: string) => {
      if (downloadingIdsRef.current.has(episodeId)) return;
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
      progressMap,
      startBatchDownload,
      startEpisodeDownload,
    }),
    [
      batchProgress,
      isBatchDownloading,
      downloadingIds,
      progressMap,
      startBatchDownload,
      startEpisodeDownload,
    ],
  );

  return <DownloadContext.Provider value={value}>{children}</DownloadContext.Provider>;
}

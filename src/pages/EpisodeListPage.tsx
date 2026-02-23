import { useState, useMemo } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { useQueryClient } from "@tanstack/react-query";
import { RefreshCw, Download, Loader2 } from "lucide-react";
import { toast } from "sonner";

import type { BatchDownloadProgress, DownloadProgress } from "@/types";
import { downloadEpisode } from "@/services/download";
import { batchDownloadNew } from "@/services/download";
import { usePodcasts } from "@/hooks/use-podcasts";
import { useEpisodes, useCheckNewEpisodes, episodeKeys } from "@/hooks/use-episodes";
import { Button } from "@/components/ui/button";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Header } from "@/components/common/Header";
import { StatusBar } from "@/components/common/StatusBar";
import { EpisodeCard } from "@/components/episode/EpisodeCard";

function EpisodeListPage() {
  const { id } = useParams();
  const podcastId = Number(id);
  const navigate = useNavigate();
  const queryClient = useQueryClient();

  const { data: podcasts, isSuccess: podcastsLoaded } = usePodcasts();
  const podcast = podcasts?.find((p) => p.id === podcastId);

  const { data: episodes, isLoading, error } = useEpisodes(podcastId);
  const checkNew = useCheckNewEpisodes();

  const [downloadingEpisodeId, setDownloadingEpisodeId] = useState<number | null>(null);
  const [singleProgress, setSingleProgress] = useState<DownloadProgress | null>(null);
  const [batchProgress, setBatchProgress] = useState<BatchDownloadProgress | null>(null);
  const [isBatchDownloading, setIsBatchDownloading] = useState(false);

  async function handleDownload(episodeId: number) {
    setDownloadingEpisodeId(episodeId);
    try {
      await downloadEpisode(episodeId, (progress) => {
        setSingleProgress(progress);
      });
      toast.success("ダウンロードが完了しました");
      queryClient.invalidateQueries({ queryKey: episodeKeys.list(podcastId) });
    } catch (err) {
      toast.error(String(err));
    } finally {
      setDownloadingEpisodeId(null);
      setSingleProgress(null);
    }
  }

  async function handleBatchDownload() {
    console.log("[BatchDL] 開始: podcastId=%d", podcastId);
    setIsBatchDownloading(true);
    try {
      await batchDownloadNew([podcastId], (progress) => {
        setBatchProgress(progress);
      });
      console.log("[BatchDL] 完了");
      toast.success("一括ダウンロードが完了しました");
      queryClient.invalidateQueries({ queryKey: episodeKeys.list(podcastId) });
    } catch (err) {
      console.error("[BatchDL] エラー:", err);
      toast.error(String(err));
    } finally {
      setBatchProgress(null);
      setIsBatchDownloading(false);
    }
  }

  function handleCheckNew() {
    checkNew.mutate(podcastId, {
      onSuccess: (result) => {
        if (result.newlyFoundCount > 0) {
          toast.success(`${result.newlyFoundCount} 件の新着エピソードが見つかりました`);
        } else {
          toast("新着エピソードはありません");
        }
      },
      onError: (err) => {
        toast.error(String(err));
      },
    });
  }

  const isDownloading = downloadingEpisodeId !== null || isBatchDownloading;
  const hasNewEpisodes = episodes?.some((e) => e.isNew) ?? false;

  const statusBarProgress = useMemo(() => {
    if (batchProgress) {
      return {
        type: "batch" as const,
        title: batchProgress.currentEpisodeTitle,
        percentage: batchProgress.episodeProgress.percentage ?? 0,
        completedCount: batchProgress.completedCount,
        totalCount: batchProgress.totalCount,
      };
    }
    if (singleProgress) {
      return {
        type: "single" as const,
        title: episodes?.find((e) => e.id === singleProgress.episodeId)?.title ?? "",
        percentage: singleProgress.percentage ?? 0,
      };
    }
    return null;
  }, [batchProgress, singleProgress, episodes]);

  if (podcastsLoaded && !podcast) {
    return (
      <div className="flex flex-col h-screen">
        <Header backTo="/" />
        <div className="flex-1 flex flex-col items-center justify-center gap-4">
          <p className="text-muted-foreground">番組が見つかりませんでした</p>
          <Button variant="outline" onClick={() => navigate("/")}>
            番組一覧に戻る
          </Button>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-screen">
      <Header backTo="/" />

      <main className="flex-1 overflow-auto p-4">
        {podcast && (
          <div className="flex items-center gap-3 mb-4">
            <Avatar className="h-16 w-16 shrink-0">
              {podcast.imageUrl && <AvatarImage src={podcast.imageUrl} alt={podcast.title} />}
              <AvatarFallback>{podcast.title.charAt(0)}</AvatarFallback>
            </Avatar>
            <div className="min-w-0">
              <h2 className="text-lg font-bold truncate">{podcast.title}</h2>
              {podcast.author && (
                <p className="text-sm text-muted-foreground truncate">{podcast.author}</p>
              )}
            </div>
          </div>
        )}

        <div className="mb-4 flex items-center gap-2">
          <Button
            variant="outline"
            size="sm"
            onClick={handleCheckNew}
            disabled={checkNew.isPending}
          >
            <RefreshCw className={`mr-1.5 h-4 w-4 ${checkNew.isPending ? "animate-spin" : ""}`} />
            新着をチェック
          </Button>
          <Button
            variant="outline"
            size="sm"
            onClick={handleBatchDownload}
            disabled={isDownloading || !hasNewEpisodes}
          >
            {isBatchDownloading ? (
              <Loader2 className="mr-1.5 h-4 w-4 animate-spin" />
            ) : (
              <Download className="mr-1.5 h-4 w-4" />
            )}
            新着を一括DL
          </Button>
        </div>

        {isLoading && (
          <div className="flex justify-center py-12">
            <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
          </div>
        )}

        {error && <p className="text-center py-12 text-destructive">{String(error)}</p>}

        {episodes && episodes.length === 0 && (
          <p className="text-center py-12 text-muted-foreground">エピソードがありません</p>
        )}

        {episodes && episodes.length > 0 && (
          <div className="space-y-2">
            {episodes.map((episode) => (
              <EpisodeCard
                key={episode.id}
                episode={episode}
                isDownloading={downloadingEpisodeId === episode.id}
                onDownload={() => handleDownload(episode.id)}
              />
            ))}
          </div>
        )}
      </main>

      <StatusBar progress={statusBarProgress} />
    </div>
  );
}

export default EpisodeListPage;

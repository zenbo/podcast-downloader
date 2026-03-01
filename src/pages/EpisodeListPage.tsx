import { useParams, useNavigate } from "react-router-dom";
import { RefreshCw, Download, Loader2 } from "lucide-react";
import { toast } from "sonner";

import { useDownload } from "@/stores/download-context";
import { usePodcasts } from "@/hooks/use-podcasts";
import { useEpisodes, useCheckNewEpisodes, useSkipEpisode } from "@/hooks/use-episodes";
import { Button } from "@/components/ui/button";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Header } from "@/components/common/Header";
import { EpisodeCard } from "@/components/episode/EpisodeCard";

function EpisodeListPage() {
  const { id } = useParams();
  const podcastId = Number(id);
  const navigate = useNavigate();

  const { data: podcasts, isSuccess: podcastsLoaded } = usePodcasts();
  const podcast = podcasts?.find((p) => p.id === podcastId);

  const { data: episodes, isLoading, error } = useEpisodes(podcastId);
  const checkNew = useCheckNewEpisodes();
  const skipMutation = useSkipEpisode(podcastId);

  const {
    startBatchDownload,
    startEpisodeDownload,
    isBatchDownloading,
    downloadingIds,
    batchTargetIds,
    batchProgress,
  } = useDownload();

  function handleDownload(episodeId: number) {
    const episode = episodes?.find((e) => e.id === episodeId);
    startEpisodeDownload(episodeId, episode?.title ?? "");
  }

  function handleSkip(episodeId: number) {
    skipMutation.mutate(episodeId, {
      onError: (err) => {
        toast.error(String(err));
      },
    });
  }

  function handleBatchDownload() {
    startBatchDownload([podcastId]);
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

  const isDownloading = downloadingIds.size > 0 || isBatchDownloading;
  const hasNewEpisodes = episodes?.some((e) => e.isNew) ?? false;

  if (podcastsLoaded && !podcast) {
    return (
      <>
        <Header backTo="/" />
        <div className="flex-1 flex flex-col items-center justify-center gap-4">
          <p className="text-muted-foreground">番組が見つかりませんでした</p>
          <Button variant="outline" onClick={() => navigate("/")}>
            番組一覧に戻る
          </Button>
        </div>
      </>
    );
  }

  return (
    <>
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
            {episodes.map((episode) => {
              const isBatchTarget = batchTargetIds.has(episode.id);
              const isActiveInBatch =
                isBatchTarget && batchProgress?.currentEpisodeId === episode.id;
              return (
                <EpisodeCard
                  key={episode.id}
                  episode={episode}
                  isDownloading={downloadingIds.has(episode.id) || isActiveInBatch}
                  isQueued={isBatchTarget && !isActiveInBatch && episode.downloadedAt === null}
                  isSkipping={skipMutation.isPending && skipMutation.variables === episode.id}
                  onDownload={() => handleDownload(episode.id)}
                  onSkip={() => handleSkip(episode.id)}
                />
              );
            })}
          </div>
        )}
      </main>
    </>
  );
}

export default EpisodeListPage;

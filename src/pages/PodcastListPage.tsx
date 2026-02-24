import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { Plus, Download, Loader2, RefreshCw } from "lucide-react";
import { toast } from "sonner";

import { useDownload } from "@/stores/download-context";
import { usePodcasts, useCheckAllNew } from "@/hooks/use-podcasts";
import { Button } from "@/components/ui/button";
import { Header } from "@/components/common/Header";
import { PodcastCard } from "@/components/podcast/PodcastCard";
import { RegisterPodcastDialog } from "@/components/podcast/RegisterPodcastDialog";
import { DeletePodcastDialog } from "@/components/podcast/DeletePodcastDialog";

function PodcastListPage() {
  const navigate = useNavigate();
  const { data: podcasts, isLoading, error } = usePodcasts();
  const checkAllNew = useCheckAllNew();
  const { startBatchDownload, isBatchDownloading } = useDownload();

  const [selectedIds, setSelectedIds] = useState<Set<number>>(new Set());
  const [registerDialogOpen, setRegisterDialogOpen] = useState(false);
  const [deleteTarget, setDeleteTarget] = useState<{
    id: number;
    title: string;
  } | null>(null);

  function handleCheckedChange(podcastId: number, checked: boolean) {
    setSelectedIds((prev) => {
      const next = new Set(prev);
      if (checked) {
        next.add(podcastId);
      } else {
        next.delete(podcastId);
      }
      return next;
    });
  }

  function handleBatchDownload() {
    const ids = Array.from(selectedIds);
    if (ids.length === 0) return;

    startBatchDownload(ids);
    setSelectedIds(new Set());
  }

  function handleCheckAllNew() {
    checkAllNew.mutate(undefined, {
      onSuccess: (results) => {
        const total = results.reduce((sum, r) => sum + r.newlyFoundCount, 0);
        if (total > 0) {
          toast.success(`${total} 件の新着エピソードが見つかりました`);
        } else {
          toast("新着エピソードはありません");
        }
      },
      onError: (err) => {
        toast.error(String(err));
      },
    });
  }

  return (
    <>
      <Header>
        <Button
          variant="outline"
          size="sm"
          onClick={handleCheckAllNew}
          disabled={checkAllNew.isPending}
        >
          <RefreshCw className={`mr-1.5 h-4 w-4 ${checkAllNew.isPending ? "animate-spin" : ""}`} />
          全新着チェック
        </Button>
      </Header>

      <main className="flex-1 overflow-auto p-4">
        <div className="mb-4 flex items-center justify-between">
          <Button onClick={() => setRegisterDialogOpen(true)}>
            <Plus className="mr-1.5 h-4 w-4" />
            番組を追加
          </Button>
          <Button
            variant="outline"
            onClick={handleBatchDownload}
            disabled={selectedIds.size === 0 || isBatchDownloading}
          >
            {isBatchDownloading ? (
              <Loader2 className="mr-1.5 h-4 w-4 animate-spin" />
            ) : (
              <Download className="mr-1.5 h-4 w-4" />
            )}
            選択した番組の新着をDL
          </Button>
        </div>

        {isLoading && (
          <div className="flex justify-center py-12">
            <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
          </div>
        )}

        {error && <p className="text-center py-12 text-destructive">{String(error)}</p>}

        {podcasts && podcasts.length === 0 && (
          <div className="text-center py-12 text-muted-foreground">
            <p className="mb-2">番組がまだ登録されていません</p>
            <Button variant="outline" onClick={() => setRegisterDialogOpen(true)}>
              <Plus className="mr-1.5 h-4 w-4" />
              番組を追加
            </Button>
          </div>
        )}

        {podcasts && podcasts.length > 0 && (
          <div className="space-y-2">
            {podcasts.map((podcast) => (
              <PodcastCard
                key={podcast.id}
                podcast={podcast}
                checked={selectedIds.has(podcast.id)}
                onCheckedChange={(checked) => handleCheckedChange(podcast.id, checked)}
                onDelete={() => setDeleteTarget({ id: podcast.id, title: podcast.title })}
                onNavigate={() => navigate(`/podcast/${podcast.id}`)}
              />
            ))}
          </div>
        )}
      </main>

      <RegisterPodcastDialog open={registerDialogOpen} onOpenChange={setRegisterDialogOpen} />

      <DeletePodcastDialog
        open={deleteTarget !== null}
        onOpenChange={(open) => {
          if (!open) setDeleteTarget(null);
        }}
        podcastTitle={deleteTarget?.title ?? ""}
        podcastId={deleteTarget?.id ?? 0}
      />
    </>
  );
}

export default PodcastListPage;

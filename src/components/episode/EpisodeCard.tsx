import { Download, Loader2, Check, Circle } from "lucide-react";
import type { Episode } from "@/types";
import { Card } from "@/components/ui/card";
import { Button } from "@/components/ui/button";

interface EpisodeCardProps {
  episode: Episode;
  isDownloading: boolean;
  onDownload: () => void;
}

function formatDate(iso: string): string {
  return iso.slice(0, 10);
}

export function EpisodeCard({
  episode,
  isDownloading,
  onDownload,
}: EpisodeCardProps) {
  const isDownloaded = episode.downloadedAt !== null;

  return (
    <Card className="flex items-center gap-3 p-3">
      <div className="shrink-0 w-5 flex justify-center">
        {isDownloading ? (
          <Loader2 className="h-4 w-4 animate-spin text-muted-foreground" />
        ) : isDownloaded ? (
          <Check className="h-4 w-4 text-green-600" />
        ) : (
          <Circle className="h-3 w-3 fill-blue-500 text-blue-500" />
        )}
      </div>

      <div className="min-w-0 flex-1">
        <p className="text-sm font-medium truncate">{episode.title}</p>
        <p className="text-xs text-muted-foreground">
          {formatDate(episode.publishedAt)}
        </p>
      </div>

      <Button
        variant="ghost"
        size="icon"
        aria-label="ダウンロード"
        onClick={onDownload}
        disabled={isDownloading}
      >
        {isDownloading ? (
          <Loader2 className="h-4 w-4 animate-spin" />
        ) : (
          <Download className="h-4 w-4" />
        )}
      </Button>
    </Card>
  );
}

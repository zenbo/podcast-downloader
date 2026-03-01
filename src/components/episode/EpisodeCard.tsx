import { Download, Loader2, Check, Clock } from "lucide-react";
import type { Episode } from "@/types";
import { Card } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Tooltip, TooltipTrigger, TooltipContent } from "@/components/ui/tooltip";

interface EpisodeCardProps {
  episode: Episode;
  isDownloading: boolean;
  isQueued: boolean;
  isSkipping: boolean;
  onDownload: () => void;
  onSkip: () => void;
}

function formatDate(iso: string): string {
  return iso.slice(0, 10);
}

export function EpisodeCard({
  episode,
  isDownloading,
  isQueued,
  isSkipping,
  onDownload,
  onSkip,
}: EpisodeCardProps) {
  const isDownloaded = episode.downloadedAt !== null;

  return (
    <Card className="flex-row items-center gap-3 p-3">
      <div className="shrink-0 w-5 flex justify-center">
        {isDownloading ? (
          <Loader2 className="h-4 w-4 animate-spin text-muted-foreground" />
        ) : isQueued ? (
          <Clock className="h-4 w-4 text-muted-foreground" />
        ) : isDownloaded ? (
          <Check className="h-4 w-4 text-green-600" />
        ) : episode.isNew ? (
          <span className="h-2.5 w-2.5 rounded-full bg-blue-500" />
        ) : null}
      </div>

      <div className="min-w-0 flex-1">
        <p className="text-sm font-medium truncate">{episode.title}</p>
        <p className="text-xs text-muted-foreground">{formatDate(episode.publishedAt)}</p>
      </div>

      {!isDownloaded && !isDownloading && !isQueued && (
        <Tooltip>
          <TooltipTrigger asChild>
            <Button
              variant="ghost"
              size="icon"
              aria-label="DL済みにする"
              onClick={onSkip}
              disabled={isSkipping}
            >
              <Check className="h-4 w-4" />
            </Button>
          </TooltipTrigger>
          <TooltipContent>DL済みにする</TooltipContent>
        </Tooltip>
      )}

      <Tooltip>
        <TooltipTrigger asChild>
          <Button
            variant="ghost"
            size="icon"
            aria-label={isQueued ? "DL予定" : "ダウンロード"}
            onClick={onDownload}
            disabled={isDownloading || isQueued}
          >
            {isDownloading ? (
              <Loader2 className="h-4 w-4 animate-spin" />
            ) : isQueued ? (
              <Clock className="h-4 w-4" />
            ) : (
              <Download className="h-4 w-4" />
            )}
          </Button>
        </TooltipTrigger>
        <TooltipContent>{isQueued ? "DL予定" : "ダウンロード"}</TooltipContent>
      </Tooltip>
    </Card>
  );
}

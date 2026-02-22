import type { BatchDownloadProgress } from "@/types";
import { Progress } from "@/components/ui/progress";

interface StatusBarProps {
  progress: BatchDownloadProgress | null;
}

export function StatusBar({ progress }: StatusBarProps) {
  if (!progress) return null;

  const percentage = progress.episodeProgress.percentage ?? 0;

  return (
    <div className="border-t px-4 py-2">
      <div className="flex items-center justify-between text-sm text-muted-foreground mb-1">
        <span className="truncate mr-2">
          {progress.currentEpisodeTitle}
        </span>
        <span className="shrink-0">
          {progress.completedCount}/{progress.totalCount}
        </span>
      </div>
      <Progress value={percentage} />
    </div>
  );
}

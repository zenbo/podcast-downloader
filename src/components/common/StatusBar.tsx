import { Progress } from "@/components/ui/progress";

type ProgressInfo =
  | { type: "single"; title: string; percentage: number }
  | {
      type: "batch";
      title: string;
      percentage: number;
      completedCount: number;
      totalCount: number;
    };

interface StatusBarProps {
  progress: ProgressInfo | null;
}

export function StatusBar({ progress }: StatusBarProps) {
  if (!progress) return null;

  return (
    <div className="border-t px-4 py-2">
      <div className="flex items-center justify-between text-sm text-muted-foreground mb-1">
        <span className="truncate mr-2">{progress.title}</span>
        <span className="shrink-0">
          {progress.type === "batch" &&
            `${progress.completedCount}/${progress.totalCount} · `}
          {Math.round(progress.percentage)}%
        </span>
      </div>
      <Progress value={progress.percentage} />
    </div>
  );
}

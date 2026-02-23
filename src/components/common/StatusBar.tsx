import { Progress } from "@/components/ui/progress";

type ProgressInfo =
  | { type: "single"; id: number; title: string; percentage: number }
  | {
      type: "batch";
      id: number;
      title: string;
      percentage: number;
      completedCount: number;
      totalCount: number;
    };

interface StatusBarProps {
  progress: ProgressInfo | ProgressInfo[] | null;
}

export function StatusBar({ progress }: StatusBarProps) {
  if (!progress) return null;

  const items = Array.isArray(progress) ? progress : [progress];
  if (items.length === 0) return null;

  return (
    <div className="border-t px-4 py-2 space-y-2">
      {items.map((item) => (
        <div key={item.id}>
          <div className="flex items-center justify-between text-sm text-muted-foreground mb-1">
            <span className="truncate mr-2">{item.title}</span>
            <span className="shrink-0">
              {item.type === "batch" && `${item.completedCount}/${item.totalCount} · `}
              {Math.round(item.percentage)}%
            </span>
          </div>
          <Progress value={item.percentage} />
        </div>
      ))}
    </div>
  );
}

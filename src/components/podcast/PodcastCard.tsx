import { Trash2 } from "lucide-react";
import type { PodcastSummary } from "@/types";
import { Card } from "@/components/ui/card";
import { Checkbox } from "@/components/ui/checkbox";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";

interface PodcastCardProps {
  podcast: PodcastSummary;
  checked: boolean;
  onCheckedChange: (checked: boolean) => void;
  onDelete: () => void;
  onNavigate: () => void;
}

export function PodcastCard({
  podcast,
  checked,
  onCheckedChange,
  onDelete,
  onNavigate,
}: PodcastCardProps) {
  return (
    <Card className="flex items-center gap-3 p-3">
      <Checkbox
        checked={checked}
        onCheckedChange={(v) => onCheckedChange(v === true)}
      />

      <Avatar className="h-12 w-12 shrink-0">
        {podcast.imageUrl && <AvatarImage src={podcast.imageUrl} alt={podcast.title} />}
        <AvatarFallback>{podcast.title.charAt(0)}</AvatarFallback>
      </Avatar>

      <div className="min-w-0 flex-1">
        <button
          type="button"
          className="text-sm font-medium hover:underline text-left truncate block w-full"
          onClick={onNavigate}
        >
          {podcast.title}
        </button>
        {podcast.author && (
          <p className="text-xs text-muted-foreground truncate">
            {podcast.author}
          </p>
        )}
      </div>

      <div className="flex items-center gap-2 shrink-0">
        {podcast.newEpisodeCount > 0 && (
          <Badge variant="destructive">{podcast.newEpisodeCount}</Badge>
        )}
        <Button variant="ghost" size="icon" onClick={onDelete}>
          <Trash2 className="h-4 w-4" />
        </Button>
      </div>
    </Card>
  );
}

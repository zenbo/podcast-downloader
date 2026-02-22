import { useDeletePodcast } from "@/hooks/use-podcasts";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@/components/ui/alert-dialog";

interface DeletePodcastDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  podcastTitle: string;
  podcastId: number;
}

export function DeletePodcastDialog({
  open,
  onOpenChange,
  podcastTitle,
  podcastId,
}: DeletePodcastDialogProps) {
  const mutation = useDeletePodcast();

  function handleDelete() {
    mutation.mutate(podcastId, {
      onSuccess: () => {
        onOpenChange(false);
      },
    });
  }

  return (
    <AlertDialog open={open} onOpenChange={onOpenChange}>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>番組を削除</AlertDialogTitle>
          <AlertDialogDescription>
            「{podcastTitle}」を削除しますか？
            関連するエピソード情報もすべて削除されます。
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel disabled={mutation.isPending}>
            キャンセル
          </AlertDialogCancel>
          <AlertDialogAction
            onClick={handleDelete}
            disabled={mutation.isPending}
          >
            削除
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}

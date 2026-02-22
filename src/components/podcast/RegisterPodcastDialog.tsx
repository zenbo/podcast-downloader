import { useState } from "react";
import { Loader2 } from "lucide-react";
import { useRegisterPodcast } from "@/hooks/use-podcasts";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

interface RegisterPodcastDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

const PODCAST_ID_PATTERN = /id\d+/;

export function RegisterPodcastDialog({
  open,
  onOpenChange,
}: RegisterPodcastDialogProps) {
  const [url, setUrl] = useState("");
  const [validationError, setValidationError] = useState<string | null>(null);
  const mutation = useRegisterPodcast();

  function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setValidationError(null);

    const trimmed = url.trim();
    if (!trimmed) {
      setValidationError("URL を入力してください");
      return;
    }
    if (!PODCAST_ID_PATTERN.test(trimmed)) {
      setValidationError(
        "Apple Podcasts の URL を入力してください（例: https://podcasts.apple.com/.../id12345）",
      );
      return;
    }

    mutation.mutate(trimmed, {
      onSuccess: () => {
        setUrl("");
        setValidationError(null);
        onOpenChange(false);
      },
    });
  }

  function handleOpenChange(nextOpen: boolean) {
    if (!nextOpen) {
      setUrl("");
      setValidationError(null);
      mutation.reset();
    }
    onOpenChange(nextOpen);
  }

  const errorMessage =
    validationError ?? (mutation.error ? String(mutation.error) : null);

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogContent>
        <form onSubmit={handleSubmit}>
          <DialogHeader>
            <DialogTitle>番組を追加</DialogTitle>
            <DialogDescription>
              Apple Podcasts の URL を入力してください
            </DialogDescription>
          </DialogHeader>
          <div className="py-4">
            <Label htmlFor="podcast-url" className="mb-2 block">
              URL
            </Label>
            <Input
              id="podcast-url"
              type="url"
              placeholder="https://podcasts.apple.com/..."
              value={url}
              onChange={(e) => setUrl(e.target.value)}
              disabled={mutation.isPending}
            />
            {errorMessage && (
              <p className="mt-2 text-sm text-destructive">{errorMessage}</p>
            )}
          </div>
          <DialogFooter>
            <Button
              type="button"
              variant="outline"
              onClick={() => handleOpenChange(false)}
              disabled={mutation.isPending}
            >
              キャンセル
            </Button>
            <Button type="submit" disabled={mutation.isPending}>
              {mutation.isPending && (
                <Loader2 className="mr-1.5 h-4 w-4 animate-spin" />
              )}
              登録
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}

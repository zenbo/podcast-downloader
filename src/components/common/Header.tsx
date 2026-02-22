import { useNavigate } from "react-router-dom";
import { RefreshCw, Settings } from "lucide-react";
import { Button } from "@/components/ui/button";

interface HeaderProps {
  onCheckAllNew: () => void;
  isChecking: boolean;
}

export function Header({ onCheckAllNew, isChecking }: HeaderProps) {
  const navigate = useNavigate();

  return (
    <header className="flex items-center justify-between border-b px-4 py-3">
      <h1 className="text-lg font-bold">Podcast Downloader</h1>
      <div className="flex items-center gap-2">
        <Button
          variant="outline"
          size="sm"
          onClick={onCheckAllNew}
          disabled={isChecking}
        >
          <RefreshCw
            className={`mr-1.5 h-4 w-4 ${isChecking ? "animate-spin" : ""}`}
          />
          全新着チェック
        </Button>
        <Button
          variant="ghost"
          size="icon"
          onClick={() => navigate("/settings")}
        >
          <Settings className="h-5 w-5" />
        </Button>
      </div>
    </header>
  );
}

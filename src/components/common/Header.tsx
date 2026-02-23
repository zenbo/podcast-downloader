import type { ReactNode } from "react";
import { useNavigate } from "react-router-dom";
import { ArrowLeft, Settings } from "lucide-react";
import { Button } from "@/components/ui/button";

interface HeaderProps {
  backTo?: string;
  children?: ReactNode;
}

export function Header({ backTo, children }: HeaderProps) {
  const navigate = useNavigate();

  return (
    <header className="flex items-center justify-between border-b px-4 py-3">
      <div className="flex items-center gap-2">
        {backTo && (
          <Button variant="ghost" size="icon" aria-label="戻る" onClick={() => navigate(backTo)}>
            <ArrowLeft className="h-5 w-5" />
          </Button>
        )}
        <h1 className="text-lg font-bold">Podcast Downloader</h1>
      </div>
      <div className="flex items-center gap-2">
        {children}
        <Button variant="ghost" size="icon" aria-label="設定" onClick={() => navigate("/settings")}>
          <Settings className="h-5 w-5" />
        </Button>
      </div>
    </header>
  );
}

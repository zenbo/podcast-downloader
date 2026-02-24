import { useMemo } from "react";
import { BrowserRouter, Routes, Route } from "react-router-dom";

import { useDownload } from "@/stores/download-context";
import { StatusBar } from "@/components/common/StatusBar";
import type { ProgressInfo } from "@/components/common/StatusBar";
import PodcastListPage from "./pages/PodcastListPage";
import EpisodeListPage from "./pages/EpisodeListPage";
import SettingsPage from "./pages/SettingsPage";

function AppLayout() {
  const { batchProgress, progressMap } = useDownload();

  const statusBarProgress = useMemo(() => {
    const items: ProgressInfo[] = [];

    if (batchProgress) {
      items.push({
        type: "batch",
        id: batchProgress.currentEpisodeId,
        title: batchProgress.currentEpisodeTitle,
        percentage: batchProgress.episodeProgress.percentage ?? 0,
        completedCount: batchProgress.completedCount,
        totalCount: batchProgress.totalCount,
      });
    }

    for (const [id, { progress, title }] of progressMap) {
      items.push({
        type: "single",
        id,
        title,
        percentage: progress.percentage ?? 0,
      });
    }

    return items.length > 0 ? items : null;
  }, [batchProgress, progressMap]);

  return (
    <div className="flex flex-col h-screen">
      <div className="flex-1 flex flex-col overflow-hidden">
        <Routes>
          <Route path="/" element={<PodcastListPage />} />
          <Route path="/podcast/:id" element={<EpisodeListPage />} />
          <Route path="/settings" element={<SettingsPage />} />
        </Routes>
      </div>
      <StatusBar progress={statusBarProgress} />
    </div>
  );
}

function App() {
  return (
    <BrowserRouter>
      <AppLayout />
    </BrowserRouter>
  );
}

export default App;

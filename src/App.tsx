import { BrowserRouter, Routes, Route } from "react-router-dom";
import PodcastListPage from "./pages/PodcastListPage";
import EpisodeListPage from "./pages/EpisodeListPage";
import SettingsPage from "./pages/SettingsPage";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<PodcastListPage />} />
        <Route path="/podcast/:id" element={<EpisodeListPage />} />
        <Route path="/settings" element={<SettingsPage />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;

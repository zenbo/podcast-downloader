import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, waitFor, fireEvent } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { QueryClientProvider } from "@tanstack/react-query";

import PodcastListPage from "@/pages/PodcastListPage";
import { createTestQueryClient } from "@/test/test-utils";
import type { PodcastSummary, PodcastNewCount } from "@/types";

vi.mock("@/services/podcast", () => ({
  listPodcasts: vi.fn(),
  registerPodcast: vi.fn(),
  deletePodcast: vi.fn(),
}));

vi.mock("@/services/episode", () => ({
  checkAllNew: vi.fn(),
}));

vi.mock("@/stores/download-context", () => ({
  useDownload: () => ({
    startBatchDownload: vi.fn(),
    isBatchDownloading: false,
  }),
}));

import { listPodcasts } from "@/services/podcast";
import { checkAllNew } from "@/services/episode";

const mockListPodcasts = vi.mocked(listPodcasts);
const mockCheckAllNew = vi.mocked(checkAllNew);

function renderPage() {
  const queryClient = createTestQueryClient();
  return render(
    <QueryClientProvider client={queryClient}>
      <MemoryRouter>
        <PodcastListPage />
      </MemoryRouter>
    </QueryClientProvider>,
  );
}

const podcasts: PodcastSummary[] = [
  {
    id: 1,
    title: "Podcast A",
    author: null,
    imageUrl: null,
    newEpisodeCount: 0,
    latestPublishedAt: null,
  },
  {
    id: 2,
    title: "Podcast B",
    author: null,
    imageUrl: null,
    newEpisodeCount: 0,
    latestPublishedAt: null,
  },
  {
    id: 3,
    title: "Podcast C",
    author: null,
    imageUrl: null,
    newEpisodeCount: 0,
    latestPublishedAt: null,
  },
];

function getCheckedStates(): string[] {
  return screen.getAllByRole("checkbox").map((el) => el.getAttribute("data-state") ?? "");
}

describe("PodcastListPage - 全新着チェック後の自動選択", () => {
  beforeEach(() => {
    vi.resetAllMocks();
    mockListPodcasts.mockResolvedValue(podcasts);
  });

  it("newCount > 0 の番組のみが自動でチェックされる", async () => {
    const results: PodcastNewCount[] = [
      { podcastId: 1, title: "Podcast A", newCount: 3, newlyFoundCount: 3 },
      { podcastId: 2, title: "Podcast B", newCount: 0, newlyFoundCount: 0 },
      { podcastId: 3, title: "Podcast C", newCount: 2, newlyFoundCount: 0 },
    ];
    mockCheckAllNew.mockResolvedValue(results);

    renderPage();
    await screen.findByText("Podcast A");

    fireEvent.click(screen.getByRole("button", { name: /全新着チェック/ }));

    await waitFor(() => {
      expect(getCheckedStates()).toEqual(["checked", "unchecked", "checked"]);
    });
  });

  it("再チェック時は前回の選択を上書きする", async () => {
    mockCheckAllNew.mockResolvedValueOnce([
      { podcastId: 1, title: "Podcast A", newCount: 3, newlyFoundCount: 3 },
      { podcastId: 2, title: "Podcast B", newCount: 0, newlyFoundCount: 0 },
      { podcastId: 3, title: "Podcast C", newCount: 0, newlyFoundCount: 0 },
    ]);

    renderPage();
    await screen.findByText("Podcast A");

    fireEvent.click(screen.getByRole("button", { name: /全新着チェック/ }));
    await waitFor(() => {
      expect(getCheckedStates()).toEqual(["checked", "unchecked", "unchecked"]);
    });

    mockCheckAllNew.mockResolvedValueOnce([
      { podcastId: 1, title: "Podcast A", newCount: 0, newlyFoundCount: 0 },
      { podcastId: 2, title: "Podcast B", newCount: 1, newlyFoundCount: 1 },
      { podcastId: 3, title: "Podcast C", newCount: 0, newlyFoundCount: 0 },
    ]);

    fireEvent.click(screen.getByRole("button", { name: /全新着チェック/ }));
    await waitFor(() => {
      expect(getCheckedStates()).toEqual(["unchecked", "checked", "unchecked"]);
    });
  });
});

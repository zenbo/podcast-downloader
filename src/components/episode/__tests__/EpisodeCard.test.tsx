import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { EpisodeCard } from "@/components/episode/EpisodeCard";
import { TooltipProvider } from "@/components/ui/tooltip";
import type { Episode } from "@/types";

function createEpisode(overrides: Partial<Episode> = {}): Episode {
  return {
    id: 1,
    podcastId: 1,
    guid: "guid-1",
    title: "テストエピソード",
    description: null,
    audioUrl: "https://example.com/audio.mp3",
    fileSize: null,
    publishedAt: "2025-01-01T00:00:00Z",
    downloadedAt: null,
    createdAt: "2025-01-01T00:00:00Z",
    isNew: false,
    ...overrides,
  };
}

const defaultProps = {
  onDownload: vi.fn(),
  onSkip: vi.fn(),
  isDownloading: false,
  isQueued: false,
  isSkipping: false,
};

function renderCard(props: Partial<Parameters<typeof EpisodeCard>[0]> = {}) {
  return render(
    <TooltipProvider>
      <EpisodeCard
        episode={createEpisode(props.episode as Partial<Episode>)}
        {...defaultProps}
        {...props}
      />
    </TooltipProvider>,
  );
}

describe("EpisodeCard", () => {
  it("通常状態: DL ボタンが有効で aria-label が「ダウンロード」", () => {
    renderCard();

    const dlButton = screen.getByRole("button", { name: "ダウンロード" });
    expect((dlButton as HTMLButtonElement).disabled).toBe(false);
  });

  it("DL 中: スピナーが表示されボタンが disabled", () => {
    renderCard({ isDownloading: true });

    const dlButton = screen.getByRole("button", { name: "ダウンロード" });
    expect((dlButton as HTMLButtonElement).disabled).toBe(true);

    // Loader2 の animate-spin クラスが存在する
    const spinner = dlButton.querySelector(".animate-spin");
    expect(spinner).not.toBeNull();
  });

  it("DL 予定: スピナーなし・ボタン disabled・aria-label が「DL予定」", () => {
    renderCard({ isQueued: true });

    const dlButton = screen.getByRole("button", { name: "DL予定" });
    expect((dlButton as HTMLButtonElement).disabled).toBe(true);

    // スピナーが表示されていないことを確認
    const spinner = dlButton.querySelector(".animate-spin");
    expect(spinner).toBeNull();
  });

  it("DL 予定時にスキップボタンが非表示", () => {
    renderCard({ isQueued: true });

    const skipButton = screen.queryByRole("button", { name: "DL済みにする" });
    expect(skipButton).toBeNull();
  });

  it("配信日に曜日が表示される", () => {
    // 2025-01-01 は水曜日
    renderCard({ episode: createEpisode({ publishedAt: "2025-01-01T00:00:00Z" }) });
    expect(screen.getByText("2025-01-01(水)")).toBeTruthy();
  });

  it("DL 済み: チェックアイコン表示・スキップボタン非表示", () => {
    renderCard({
      episode: createEpisode({ downloadedAt: "2025-01-02T00:00:00Z" }),
    });

    // スキップボタンが非表示
    const skipButton = screen.queryByRole("button", { name: "DL済みにする" });
    expect(skipButton).toBeNull();

    // 左側インジケーターにチェックアイコン（text-green-600）が表示される
    const indicator = document.querySelector(".text-green-600");
    expect(indicator).not.toBeNull();
  });
});

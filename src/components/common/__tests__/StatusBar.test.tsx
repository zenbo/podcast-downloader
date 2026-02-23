import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { StatusBar } from "@/components/common/StatusBar";

describe("StatusBar", () => {
  it("progress が null のとき何も描画しない", () => {
    const { container } = render(<StatusBar progress={null} />);
    expect(container.firstChild).toBeNull();
  });

  it("空配列のとき何も描画しない", () => {
    const { container } = render(<StatusBar progress={[]} />);
    expect(container.firstChild).toBeNull();
  });

  it("single タイプの進捗を1件表示する", () => {
    render(<StatusBar progress={{ type: "single", id: 1, title: "Episode A", percentage: 45 }} />);
    expect(screen.getByText("Episode A")).toBeDefined();
    expect(screen.getByText("45%")).toBeDefined();
  });

  it("batch タイプの進捗を表示し、完了件数/総件数を含む", () => {
    render(
      <StatusBar
        progress={{
          type: "batch",
          id: 10,
          title: "Episode B",
          percentage: 70,
          completedCount: 2,
          totalCount: 5,
        }}
      />,
    );
    expect(screen.getByText("Episode B")).toBeDefined();
    expect(screen.getByText("2/5 · 70%")).toBeDefined();
  });

  it("配列で複数の進捗を同時に表示する", () => {
    render(
      <StatusBar
        progress={[
          { type: "single", id: 1, title: "Episode X", percentage: 30 },
          { type: "single", id: 2, title: "Episode Y", percentage: 80 },
        ]}
      />,
    );
    expect(screen.getByText("Episode X")).toBeDefined();
    expect(screen.getByText("30%")).toBeDefined();
    expect(screen.getByText("Episode Y")).toBeDefined();
    expect(screen.getByText("80%")).toBeDefined();
  });

  it("percentage を小数から整数に丸めて表示する", () => {
    render(
      <StatusBar progress={{ type: "single", id: 1, title: "Episode C", percentage: 33.7 }} />,
    );
    expect(screen.getByText("34%")).toBeDefined();
  });

  it("2件中1件が完了しても残りの進捗バーが維持される", () => {
    const { rerender } = render(
      <StatusBar
        progress={[
          { type: "single", id: 1, title: "Episode A", percentage: 100 },
          { type: "single", id: 2, title: "Episode B", percentage: 50 },
        ]}
      />,
    );
    expect(screen.getByText("Episode A")).toBeDefined();
    expect(screen.getByText("Episode B")).toBeDefined();

    // Episode A が完了し、配列から除外される
    rerender(
      <StatusBar progress={[{ type: "single", id: 2, title: "Episode B", percentage: 55 }]} />,
    );
    expect(screen.queryByText("Episode A")).toBeNull();
    expect(screen.getByText("Episode B")).toBeDefined();
    expect(screen.getByText("55%")).toBeDefined();
  });
});

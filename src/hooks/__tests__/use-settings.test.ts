import { describe, it, expect, vi, beforeEach } from "vitest";
import { waitFor } from "@testing-library/react";
import { renderHookWithQueryClient } from "@/test/test-utils";
import { useSettings, useUpdateSettings } from "@/hooks/use-settings";
import type { AppSettings } from "@/types";

vi.mock("@/services/settings", () => ({
  getSettings: vi.fn(),
  updateSettings: vi.fn(),
}));

import { getSettings, updateSettings } from "@/services/settings";

const mockGetSettings = vi.mocked(getSettings);
const mockUpdateSettings = vi.mocked(updateSettings);

describe("use-settings hooks", () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  describe("useSettings", () => {
    it("getSettings を呼び出してデータを返す", async () => {
      const mockSettings: AppSettings = {
        downloadDir: "/test/downloads",
        characterReplacements: [],
        fallbackReplacement: "_",
      };
      mockGetSettings.mockResolvedValue(mockSettings);

      const { result } = renderHookWithQueryClient(() => useSettings());

      await waitFor(() => expect(result.current.isSuccess).toBe(true));
      expect(result.current.data).toEqual(mockSettings);
      expect(mockGetSettings).toHaveBeenCalledOnce();
    });
  });

  describe("useUpdateSettings", () => {
    it("updateSettings を呼び出し、成功時に settings クエリを無効化する", async () => {
      mockUpdateSettings.mockResolvedValue(undefined);

      const { result, queryClient } = renderHookWithQueryClient(() =>
        useUpdateSettings(),
      );
      const invalidateSpy = vi.spyOn(queryClient, "invalidateQueries");

      const newSettings: AppSettings = {
        downloadDir: "/new/path",
        characterReplacements: [{ before: ":", after: "-" }],
        fallbackReplacement: "-",
      };
      result.current.mutate(newSettings);

      await waitFor(() => expect(result.current.isSuccess).toBe(true));
      expect(mockUpdateSettings).toHaveBeenCalledWith(newSettings);
      expect(invalidateSpy).toHaveBeenCalledWith({
        queryKey: ["settings"],
      });
    });
  });
});

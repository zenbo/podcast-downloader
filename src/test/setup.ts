import { vi, afterEach } from "vitest";
import { cleanup } from "@testing-library/react";

afterEach(() => {
  cleanup();
});

// @tauri-apps/api/core のグローバルモック
// 実際の Channel は window.__TAURI_INTERNALS__ に依存するため、
// テスト環境では簡略化したモックを使用する
class MockChannel<T = unknown> {
  id: number;
  private _onmessage: ((response: T) => void) | undefined;

  constructor() {
    this.id = Math.floor(Math.random() * 100000);
  }

  set onmessage(handler: (response: T) => void) {
    this._onmessage = handler;
  }

  get onmessage(): (response: T) => void {
    return this._onmessage!;
  }

  toJSON(): string {
    return `__CHANNEL__:${this.id}`;
  }
}

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
  Channel: MockChannel,
}));

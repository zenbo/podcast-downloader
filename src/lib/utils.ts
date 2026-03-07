import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

const WEEKDAYS = ["日", "月", "火", "水", "木", "金", "土"] as const;

/** ISO 8601 日時文字列を "YYYY-MM-DD(曜)" 形式にフォーマットする */
export function formatDate(iso: string): string {
  const date = new Date(iso);
  const y = date.getFullYear();
  const m = String(date.getMonth() + 1).padStart(2, "0");
  const d = String(date.getDate()).padStart(2, "0");
  const w = WEEKDAYS[date.getDay()];
  return `${y}-${m}-${d}(${w})`;
}

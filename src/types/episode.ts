/** データベースから取得した完全なエピソード情報 */
export interface Episode {
  id: number;
  podcastId: number;
  guid: string;
  title: string;
  description: string | null;
  audioUrl: string;
  duration: string | null;
  fileSize: number | null;
  publishedAt: string;
  downloadedAt: string | null;
  createdAt: string;
}

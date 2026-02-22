/** 文字置換ルール */
export interface CharacterReplacement {
  before: string;
  after: string;
}

/** アプリケーション全体の設定 */
export interface AppSettings {
  downloadDir: string | null;
  characterReplacements: CharacterReplacement[];
  fallbackReplacement: string;
}

use serde::{Deserialize, Serialize};

/// アプリケーション全体の設定
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    /// ダウンロード先ベースフォルダ
    pub download_dir: Option<String>,
    /// 文字置換ルール（配列の順序で適用される）
    pub character_replacements: Vec<CharacterReplacement>,
    /// OS 禁止文字のフォールバック置換文字
    pub fallback_replacement: String,
}

/// 文字置換ルール
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterReplacement {
    pub before: String,
    pub after: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            download_dir: None,
            character_replacements: vec![
                CharacterReplacement { before: "/".to_string(), after: "-".to_string() },
                CharacterReplacement { before: ":".to_string(), after: "-".to_string() },
                CharacterReplacement { before: "?".to_string(), after: "".to_string() },
                CharacterReplacement { before: "\"".to_string(), after: "".to_string() },
                CharacterReplacement { before: "<".to_string(), after: "".to_string() },
                CharacterReplacement { before: ">".to_string(), after: "".to_string() },
                CharacterReplacement { before: "|".to_string(), after: "".to_string() },
            ],
            fallback_replacement: "_".to_string(),
        }
    }
}

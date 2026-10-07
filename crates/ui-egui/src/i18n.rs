//! Interface translations. Command ids, document text and file names remain stable.
//! Untranslated labels fall back to English so coverage can grow incrementally.

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[default]
    En,
    Ja,
    // AI編輯：新增繁體中文（zh-Hant）介面語言。
    #[serde(rename = "zh-hant")]
    ZhHant,
}

impl Language {
    pub const ALL: [Self; 3] = [Self::En, Self::Ja, Self::ZhHant];

    pub fn name(self) -> &'static str {
        match self {
            Self::En => "English",
            Self::Ja => "日本語",
            // AI編輯：新增語言名稱。
            Self::ZhHant => "繁體中文",
        }
    }

    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "en" => Some(Self::En),
            "ja" => Some(Self::Ja),
            // AI編輯：接受「zh-hant / zh-tw / zh」三種代碼。
            "zh-hant" | "zh-tw" | "zh" => Some(Self::ZhHant),
            _ => None,
        }
    }

    pub fn tr(self, text: &str) -> &str {
        match self {
            Self::Ja => {
                if let Some((_, japanese)) = JAPANESE.iter().find(|(english, _)| *english == text) {
                    return japanese;
                }
            }
            // AI編輯：新增繁體中文翻譯查詢。
            Self::ZhHant => {
                if let Some((_, traditional)) = TRADITIONAL_CHINESE.iter().find(|(english, _)| *english == text) {
                    return traditional;
                }
            }
            Self::En => {}
        }
        text
    }
}

const JAPANESE: &[(&str, &str)] = &[
    ("Menu", "メニュー"),
    ("File", "ファイル"),
    ("Edit", "編集"),
    ("Pages", "ページ"),
    ("View", "表示"),
    ("Help", "ヘルプ"),
    ("Preferences", "環境設定"),
    ("Preferences…", "環境設定…"),
    ("Interface language", "表示言語"),
    ("Identity", "個人情報"),
    ("Name on new comments", "新しい注釈の作成者名"),
    ("Open…", "開く…"),
    ("New blank PDF", "空白の PDF を作成"),
    ("Create PDF from file…", "ファイルから PDF を作成…"),
    ("Create PDF from images…", "画像から PDF を作成…"),
    ("Create PDF from clipboard", "クリップボードから PDF を作成"),
    ("Combine files…", "ファイルを結合…"),
    ("Save", "保存"),
    ("Save as…", "別名で保存…"),
    ("Close file", "ファイルを閉じる"),
    ("Close all", "すべて閉じる"),
    ("Revert", "保存済みの状態に戻す"),
    ("Print…", "印刷…"),
    ("Document properties…", "文書のプロパティ…"),
    ("Undo", "取り消し"),
    ("Redo", "やり直し"),
    ("Find…", "検索…"),
    ("Advanced search…", "高度な検索…"),
    ("Copy pages", "ページをコピー"),
    ("Cut pages", "ページを切り取り"),
    ("Paste pages", "ページを貼り付け"),
    ("Fit visible", "表示範囲に合わせる"),
    ("Marquee zoom", "範囲指定ズーム"),
    ("Take a snapshot", "スナップショットを作成"),
    ("Full screen mode", "全画面表示"),
    ("Read mode", "閲覧モード"),
    ("Switch light / dark theme", "明るい／暗いテーマを切り替え"),
    ("Comments panel", "コメントパネル"),
    ("Form fields panel", "フォームフィールドパネル"),
    ("Clear form", "フォームをクリア"),
    ("Find tools and commands…", "ツールとコマンドを検索…"),
    ("Zoom", "ズーム"),
    ("Actual size", "実際のサイズ"),
    ("Zoom to page level", "ページ全体を表示"),
    ("Fit to width", "幅に合わせる"),
    ("Display theme", "表示テーマ"),
    ("Side panels", "サイドパネル"),
    ("Enable Acrobat JavaScript", "Acrobat JavaScript を有効にする"),
    ("OK", "OK"),
];

// AI編輯：繁體中文（zh-Hant）翻譯目錄，由 AI 依英文原意翻譯（台灣慣用術語）。
const TRADITIONAL_CHINESE: &[(&str, &str)] = &[
    ("Menu", "選單"),
    ("File", "檔案"),
    ("Edit", "編輯"),
    ("Pages", "頁面"),
    ("View", "檢視"),
    ("Help", "說明"),
    ("Preferences", "偏好設定"),
    ("Preferences…", "偏好設定…"),
    ("Interface language", "介面語言"),
    ("Identity", "身分"),
    ("Name on new comments", "新註解上的名稱"),
    ("Open…", "開啟…"),
    ("New blank PDF", "新增空白 PDF"),
    ("Create PDF from file…", "從檔案建立 PDF…"),
    ("Create PDF from images…", "從影像建立 PDF…"),
    ("Create PDF from clipboard", "從剪貼簿建立 PDF"),
    ("Combine files…", "合併檔案…"),
    ("Save", "儲存"),
    ("Save as…", "另存新檔…"),
    ("Close file", "關閉檔案"),
    ("Close all", "全部關閉"),
    ("Revert", "還原為已儲存狀態"),
    ("Print…", "列印…"),
    ("Document properties…", "文件屬性…"),
    ("Undo", "復原"),
    ("Redo", "重做"),
    ("Find…", "尋找…"),
    ("Advanced search…", "進階搜尋…"),
    ("Copy pages", "複製頁面"),
    ("Cut pages", "剪下頁面"),
    ("Paste pages", "貼上頁面"),
    ("Fit visible", "符合可見範圍"),
    ("Marquee zoom", "範圍縮放"),
    ("Take a snapshot", "建立快照"),
    ("Full screen mode", "全螢幕模式"),
    ("Read mode", "閱讀模式"),
    ("Switch light / dark theme", "切換淺色／深色主題"),
    ("Comments panel", "註解面板"),
    ("Form fields panel", "表單欄位面板"),
    ("Clear form", "清除表單"),
    ("Find tools and commands…", "尋找工具與命令…"),
    ("Zoom", "縮放"),
    ("Actual size", "實際大小"),
    ("Zoom to page level", "縮放至頁面大小"),
    ("Fit to width", "符合寬度"),
    ("Display theme", "顯示主題"),
    ("Side panels", "側邊面板"),
    ("Enable Acrobat JavaScript", "啟用 Acrobat JavaScript"),
    ("OK", "確定"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translations_are_unique_and_preserve_unknown_text() {
        for (i, (en, ja)) in JAPANESE.iter().enumerate() {
            assert!(!ja.is_empty());
            assert!(JAPANESE.iter().take(i).all(|(other, _)| en != other));
            assert_eq!(Language::En.tr(en), *en);
        }
        // AI編輯：繁中目錄同樣需要唯一且非空。
        for (i, (en, zh)) in TRADITIONAL_CHINESE.iter().enumerate() {
            assert!(!zh.is_empty());
            assert!(TRADITIONAL_CHINESE.iter().take(i).all(|(other, _)| en != other));
            assert_eq!(Language::En.tr(en), *en);
        }
        assert_eq!(Language::Ja.tr("File"), "ファイル");
        assert_eq!(Language::Ja.tr("日本語の文書.pdf"), "日本語の文書.pdf");
        assert_eq!(Language::ZhHant.tr("File"), "檔案");
        assert_eq!(Language::ZhHant.tr("日本語の文書.pdf"), "日本語の文書.pdf");
        assert_eq!(Language::parse("xx"), None);
        assert_eq!(Language::parse("zh-hant"), Some(Language::ZhHant));
    }

    #[test]
    fn language_persists_and_invalid_input_keeps_current_language() {
        let mut app = crate::PrintCraftApp::default();
        app.set_option("language", "ja").unwrap();
        assert_eq!(app.language, Language::Ja);
        // AI編輯：繁中語言同樣可設定並持久化。
        app.set_option("language", "zh-hant").unwrap();
        assert_eq!(app.language, Language::ZhHant);
        assert!(app.set_option("language", "xx").is_err());
        assert_eq!(app.language, Language::ZhHant);
        let mut restored = crate::PrintCraftApp::default();
        restored.restore(&app.persist());
        assert_eq!(restored.language, Language::ZhHant);
        restored.restore(r#"{"language":"xx"}"#);
        assert_eq!(restored.language, Language::ZhHant);
        let mut legacy = crate::PrintCraftApp::default();
        legacy.restore("{}");
        assert_eq!(legacy.language, Language::En);
    }
}

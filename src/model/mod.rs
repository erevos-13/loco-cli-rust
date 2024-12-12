pub mod loco {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct Author {
        pub id: u32,
        pub name: String,
        pub email: String,
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct Flagger {
        id: u32,
        name: String,
        email: String,
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct Locale {
        code: String,
        pub name: String,
        pub source: bool,
        pub plurals: Plurals,
        pub progress: Progress,
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct Plurals {
        length: u32,
        equation: String,
        forms: Vec<String>,
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct Progress {
        translated: u32,
        untranslated: u32,
        flagged: u32,
        words: u32,
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct Plural {
        id: String,
        status: String,
        flagged: bool,
        translated: bool,
        translation: String,
        revision: u32,
        comments: u32,
        modified: String,
        author: Author,
        flagger: Flagger,
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct Translation {
        id: String,
        status: String,
        flagged: bool,
        translated: bool,
        translation: String,
        revision: u32,
        comments: u32,
        modified: String,
        author: Author,
        flagger: Flagger,
        locale: Locale,
        plurals: Vec<Plural>,
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct ImportResponse {
        pub status: u16,          // Status code (200 or 201)
        pub message: String,      // Summary of import result
        pub locales: Vec<Locale>, // List of locales
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct LocaleInfo {
        pub code: String,
        pub name: String,
        #[serde(default)]
        pub source: bool,
        #[serde(default)]
        pub native: bool,
        pub plurals: PluralInfo,
        pub progress: ProgressInfo,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PluralInfo {
        pub length: u8,
        pub equation: String,
        pub forms: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ProgressInfo {
        pub translated: u32,
        pub untranslated: u32,
        pub flagged: u32,
        pub words: u32,
    }
}

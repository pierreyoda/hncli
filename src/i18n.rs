use chrono::{DateTime, Utc};

use crate::api::types::HnItemIdScalar;

pub mod english;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TranslationKey<'a> {
    // Common
    Loading,
    // Home screen stories list
    HomeErrorFlash,
    // Home screen sorting options
    HomeSortBlockTitle,
    HomeSortNew,
    HomeSortTop,
    HomeSortBest,
    // Navigation bar
    NavbarBlockTitle,
    NavbarHome,
    NavbarAskHN,
    NavbarShowHN,
    NavbarJobs,
    NavbarSettings,
    NavbarHelp,
    NavbarResume,
    // Item details
    ItemDetailsMeta {
        score: u32,
        by: &'a str,
        posted_at: &'a DateTime<Utc>,
    },
    ItemDetailsCommentsCount {
        count: u32,
    },
    // Item summary
    ItemSummaryParentCommentBy {
        parent_comment_by: &'a str,
    },
    ItemSummarySubCommentLevel {
        level: usize,
    },
    // Item comments
    ItemCommentsFetchError,
    ItemCommentsError,
    ItemCommentsNoComments,
    ItemCommentsMeta {
        index: usize,
        total: usize,
        kids: usize,
    },
    ItemCommentsLevelIndex,
    // Resume reading tab
    ItemResumeFetchError,
    ItemResumeError,
    ItemResumeNoItems,
    ItemResumeLastRead,
    // User Profile
    UserProfileError,
    UserProfileFetchError {
        user_id: HnItemIdScalar,
    },
    UserProfileCreatedAt {
        created_at: DateTime<Utc>,
    },
    UserProfileKarma {
        karma: u32,
    },
    UserProfileAbout,
    // Settings
    SettingsScreenTitle,
    SettingsEnabled,
    SettingsDisabled,
    SettingsTheme,
    SettingsThemeBlue,
    SettingsThemeMagenta,
    SettingsThemeYellow,
    SettingsHomeDisplayMetadata,
    SettingsItemDisplayCommentsDefault,
    SettingsShowContextualHelp,
    SettingsGlobalQuitShortcut,
    SettingsSavedFlash,
    // Help
    HelpMultilineText,
    // Contextual Help
    ContextualHelpEscapeKey,
    ContextualHelpEnterKey,
    ContextualHelpBackspaceKey,
    ContextualHelpTabKey,
    ContextualHelpUpKey,
    ContextualHelpDownKey,
    ContextualHelpLeftKey,
    ContextualHelpRightKey,
}

#[derive(Hash, Debug, PartialEq, Eq)]
pub enum TranslationLanguage {
    English,
    French,
    Spanish,
}

pub const MINUTES_PER_DAY: i64 = 24 * 60;

pub fn pluralized(value: i64, word: &str) -> String {
    if value > 1 {
        format!("{value} {word}s")
    } else {
        format!("{value} {word}")
    }
}

pub trait TranslationEngine {
    fn t(key: TranslationKey) -> String;
    fn t_multiline(key: TranslationKey) -> Vec<String>;
    /// In English, formats dates just like on the official Hacker News website,
    /// for instance "June 6, 2019".
    fn t_date(date: &DateTime<Utc>) -> String;
    fn t_since(date: &DateTime<Utc>) -> String;
}

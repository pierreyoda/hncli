use chrono::{DateTime, Utc};

pub mod values;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StringKey<'a> {
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
    ItemResumeLastRead {
        since: &DateTime<Utc>,
    },
    // User Profile
    UserProfileError,
    UserProfileFetchError {
        user_id: &'a str,
    },
    UserProfileCreatedAt {
        created_at: &'a DateTime<Utc>,
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
}

pub trait StringValuesProvider: Send + Sync {
    fn v(&self, key: StringKey) -> String;
    fn v_multiline(&self, key: StringKey) -> Vec<String>;
    /// Formats dates just like on the official Hacker News website, for instance "June 6, 2019".
    fn date(&self, date: &DateTime<Utc>) -> String;
    fn since(&self, date: &DateTime<Utc>) -> String;
}

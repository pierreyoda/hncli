use chrono::{DateTime, Utc};
use log::warn;

use crate::app::strings::{StringKey, StringValuesProvider};

pub const MINUTES_PER_DAY: i64 = 24 * 60;

pub fn pluralized(value: i64, word: &str) -> String {
    if value > 1 {
        format!("{value} {word}s")
    } else {
        format!("{value} {word}")
    }
}

#[derive(Debug)]
pub struct StringValuesProviderEnglish;

impl StringValuesProvider for StringValuesProviderEnglish {
    fn v(&self, key: StringKey) -> String {
        use StringKey::*;
        match key {
    // Common
    Loading => "Loading".into(),
    // Home screen stories list
    HomeErrorFlash => "Could not fetch HackerNews stories.".into(),
    // Home screen sorting options
    HomeSortBlockTitle,
    HomeSortNew,
    HomeSortTop,
    HomeSortBest,
    // Navigation bar
    NavbarBlockTitle => "Menu".into(),
    NavbarHome => "Home".into(),
    NavbarAskHN => "Ask HN".into(),
    NavbarShowHN => "Show HN".into(),
    NavbarJobs => "Jobs".into(),
    NavbarSettings => "Settings".into(),
    NavbarHelp => "Help".into(),
    NavbarResume => "Resume".into(),
    // Item details
    ItemDetailsMeta { score, by, posted_at } => format!("{score} points by {by} {}", Self::t_since(posted_at)),
    ItemDetailsCommentsCount { count } => format!("{count} comments"),
    // Item summary
    ItemSummaryParentCommentBy { parent_comment_by
     } => format!("Parent comment by: {parent_comment_by}"),
    ItemSummarySubCommentLevel { level } => format!("Sub-comment level: {level"),
    // Item comments
    ItemCommentsFetchError => "Comments fetching issue. Please retry later.".into(),
    ItemCommentsError => "An error has occurred on this thread. Please retry later.".into(),
    ItemCommentsNoComments => "No comments yet.".into(),
    ItemCommentsMeta { index, total, kids } => format!(
        "Comment {} / {} | {}", index + 1, total, pluralized(kids as i64, "sub-comment")
    ),
    ItemCommentsLevelIndex,
    // Resume reading tab
    ItemResumeFetchError => "Could not open this Item, it may no longer be available.".into(),
    ItemResumeError => "Could not open this Item, it may no longer be available.".into(),
    ItemResumeNoItems => "No items in history.".into(),
    ItemResumeLastRead {since}=> format!("last read {}", self.since(since)),
    // User Profile
    UserProfileError => "Sorry, this user cannot be displayed due to an error.".into(),
    UserProfileFetchError { user_id } => format!("The user data of '{user_id}' cannot be loaded, please retry later."),
    UserProfileCreatedAt { created_at } => format("Created: {}", self.date(created_at)),
    UserProfileKarma { karma: u32 } => format!("Karma: {karma}"),
    UserProfileAbout => "About:".into(),
    // Settings
    SettingsScreenTitle => "Settings".into(),
    SettingsEnabled => "Enabled".into(),
    SettingsDisabled => "Disabled".into(),
    SettingsTheme => "Application-wide theme".into(),
    SettingsThemeBlue => "Blue".into(),
    SettingsThemeMagenta => "Magenta".into(),
    SettingsThemeYellow => "Yellow".into(),
    SettingsHomeDisplayMetadata => "Display the stories' metadata on main screen:".into(),
    SettingsItemDisplayCommentsDefault => "Display the comments panel by default:".into(),
    SettingsShowContextualHelp => "Show the global contextual help:".into(),
    SettingsGlobalQuitShortcut => "Enable the global 'q' quit shortcut in sub-screens, besides CTRL+C:".into(),
    SettingsSavedFlash => "Settings successfully saved.".into(),
    // Help
    HelpMultilineText => {
        warn!("i18n.t(english): no single-line translation for key: {key}");
        "(translation error)".into()
    },
    // Contextual Help
    ContextualHelpEscapeKey => "escape".into(),
    ContextualHelpEnterKey => "enter".into(),
    ContextualHelpBackspaceKey => "backspace".into(),
    ContextualHelpTabKey => "tab".into(),
    ContextualHelpUpKey => "up".into(),
    ContextualHelpDownKey => "down".into(),
    ContextualHelpLeftKey => "left".into(),
    ContextualHelpRightKey => "right".into(),
}}

    fn v_multiline(&self, key: StringKey) -> Vec<String> {
        use StringKey::*;
        match key {
            HelpMultilineText => todo!(),
            _ => {
                warn!("i18n.t_multiline(english): no multiline translation for key: {:?}", key);
                vec![]
            }
        }
    }

    fn date(&self, date: &DateTime<Utc>) -> String {
        date.format("%B %d, %Y").to_string()
    }

    fn since(&self, date: &DateTime<Utc>) -> String {
        let now = Utc::now();
        let minutes = (now - *date).num_minutes();
        match minutes {
            _ if minutes >= MINUTES_PER_DAY => {
                format!("{} ago", pluralized(minutes / MINUTES_PER_DAY, "day"))
            }
            _ if minutes >= 60 => format!("{} ago", pluralized(minutes / 60, "hour")),
            _ => format!("{} ago", pluralized(minutes, "minute")),
        }
    }
}

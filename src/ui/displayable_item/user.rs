use chrono::{DateTime, Utc};

use crate::{
    api::types::{HnItemIdScalar, HnUser},
    errors::{HnCliError, Result},
    ui::utils::datetime_from_hn_time,
};

/// Display-ready Hacker News user data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DisplayableHackerNewsUser {
    /// **Case-sensitive**, unique username.
    pub id: String,
    /// Registration date.
    pub created_at: DateTime<Utc>,
    /// Registration date, formatted for display.
    ///
    /// Format: just like on the official Hacker News website, for example "June 6, 2019".
    pub created_at_formatted: String,
    /// Total karma of the user.
    pub karma: u32,
    /// *HTML* description of the user, if any.
    pub about: Option<String>,
    /// IDs of the user's submitted items.
    pub submitted: Vec<HnItemIdScalar>,
}

impl DisplayableHackerNewsUser {
    pub fn get_hacker_news_link(&self) -> String {
        Self::build_hacker_news_link(&self.id)
    }

    pub fn build_hacker_news_link(user_id: &str) -> String {
        format!("https://news.ycombinator.com/user?id={user_id}")
    }
}

impl TryFrom<HnUser> for DisplayableHackerNewsUser {
    type Error = HnCliError;

    fn try_from(value: HnUser) -> Result<Self> {
        let created_at = datetime_from_hn_time(value.created)?;
        let created_at_formatted = created_at.format("%B %d, %Y").to_string();
        Ok(Self {
            id: value.id,
            created_at,
            created_at_formatted,
            karma: value.karma,
            about: value.about,
            submitted: value.submitted,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::api::types::HnUser;

    use super::DisplayableHackerNewsUser;

    #[test]
    fn test_user_data_parsing_and_displayable_conversion() {
        // NB: truncated "submitted"
        let json = r#"{
            "about" : "&quot;<i>Conflict is essential to human life, whether between different aspects of oneself, between oneself and the environment, between different individuals or between different groups. It follows that the aim of healthy living is not the direct elimination of conflict, which is possible only by forcible suppression of one or other of its antagonistic components, but the toleration of it—the capacity to bear the tensions of doubt and of unsatisfied need and the willingness to hold judgement in suspense until finer and finer solutions can be discovered which integrate more and more the claims of both sides. It is the psychologist&#x27;s job to make possible the acceptance of such an idea so that the richness of the varieties of experience, whether within the unit of the single personality or in the wider unit of the group, can come to expression.</i>&quot;<p>Marion Milner, &#x27;The Toleration of Conflict&#x27;, <i>Occupational Psychology</i>, 17, 1, January 1943<p>---<p>Please send HN questions to hn@ycombinator.com.",
            "created" : 1187454947,
            "id" : "dang",
            "karma" : 58042,
            "submitted": [34214712, 34214708, 34214694]
        }"#;
        let user: HnUser = serde_json::from_str(json).unwrap();
        let displayable_user: DisplayableHackerNewsUser = user.try_into().unwrap();
        assert_eq!(displayable_user.id, "dang");
        assert_eq!(displayable_user.karma, 58042);
        assert_eq!(displayable_user.created_at_formatted, "August 18, 2007");
        assert_eq!(displayable_user.about, Some("&quot;<i>Conflict is essential to human life, whether between different aspects of oneself, between oneself and the environment, between different individuals or between different groups. It follows that the aim of healthy living is not the direct elimination of conflict, which is possible only by forcible suppression of one or other of its antagonistic components, but the toleration of it—the capacity to bear the tensions of doubt and of unsatisfied need and the willingness to hold judgement in suspense until finer and finer solutions can be discovered which integrate more and more the claims of both sides. It is the psychologist&#x27;s job to make possible the acceptance of such an idea so that the richness of the varieties of experience, whether within the unit of the single personality or in the wider unit of the group, can come to expression.</i>&quot;<p>Marion Milner, &#x27;The Toleration of Conflict&#x27;, <i>Occupational Psychology</i>, 17, 1, January 1943<p>---<p>Please send HN questions to hn@ycombinator.com.".into()));
        assert_eq!(
            displayable_user.get_hacker_news_link(),
            "https://news.ycombinator.com/user?id=dang"
        );
    }
}

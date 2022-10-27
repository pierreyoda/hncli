use chrono::{DateTime, Utc};

use crate::{
    api::types::{HnItem, HnItemIdScalar},
    errors::{HnCliError, Result},
};

use super::utils::{datetime_from_hn_time, ItemWithId};

/// A display-ready Hacker News story, comment, job or poll posting.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DisplayableHackerNewsItem {
    /// Unique ID.
    pub id: HnItemIdScalar,
    /// Posted at.
    pub posted_at: DateTime<Utc>,
    /// Posted since, formatted for display.
    pub posted_since: String,
    /// Username of the poster.
    pub by_username: String,
    /// Title, if any.
    pub title: Option<String>,
    /// Text, if any.
    pub text: Option<String>,
    /// Score.
    pub score: u32,
    /// Item URL, if any.
    pub url: Option<String>,
    /// Hostname of the URL, if any.
    pub url_hostname: Option<String>,
    /// IDs of the comments on the item, if any, in ranked display order.
    pub kids: Option<Vec<HnItemIdScalar>>,
}

const MINUTES_PER_DAY: i64 = 24 * 60;

impl DisplayableHackerNewsItem {
    pub fn has_title(&self) -> bool {
        self.title.is_some()
    }

    pub fn get_hacker_news_link(&self) -> String {
        format!("https://news.ycombinator.com/item?id={}", self.id)
    }

    pub fn formatted_posted_since(posted_at: &DateTime<Utc>) -> String {
        let now = Utc::now();
        let minutes = (now - *posted_at).num_minutes();
        match minutes {
            _ if minutes >= MINUTES_PER_DAY => {
                format!("{} ago", Self::pluralized(minutes / MINUTES_PER_DAY, "day"))
            }
            _ if minutes >= 60 => format!("{} ago", Self::pluralized(minutes / 60, "hour")),
            _ => format!("{} ago", Self::pluralized(minutes, "minute")),
        }
    }

    fn pluralized(value: i64, word: &str) -> String {
        if value > 1 {
            format!("{} {}s", value, word)
        } else {
            format!("{} {}", value, word)
        }
    }
}

impl ItemWithId<HnItemIdScalar> for DisplayableHackerNewsItem {
    fn get_id(&self) -> HnItemIdScalar {
        self.id
    }
}

impl TryFrom<HnItem> for DisplayableHackerNewsItem {
    type Error = HnCliError;

    fn try_from(value: HnItem) -> Result<Self> {
        match value {
            HnItem::Story(story) => {
                let posted_at = datetime_from_hn_time(story.time);
                Ok(Self {
                    id: story.id,
                    posted_at,
                    posted_since: Self::formatted_posted_since(&posted_at),
                    by_username: story.by,
                    title: Some(story.title),
                    text: story.text,
                    score: story.score,
                    url: story.url.clone(),
                    url_hostname: story.url.map(|url| {
                        url::Url::parse(&url[..])
                            .map_err(HnCliError::UrlParsingError)
                            .expect("story URL parsing error") // TODO: avoid expect here
                            .host_str()
                            .expect("story URL must have an hostname")
                            .to_owned()
                    }),
                    kids: story.kids,
                })
            }
            HnItem::Comment(comment) => {
                let posted_at = datetime_from_hn_time(comment.time);
                Ok(Self {
                    id: comment.id,
                    posted_at,
                    posted_since: Self::formatted_posted_since(&posted_at),
                    by_username: comment.by,
                    title: None,
                    text: Some(comment.text),
                    score: comment.score.unwrap_or(0),
                    url: Some(format!(
                        "https://hacker-news.firebaseio.com/v0/item/{}.json?print=pretty",
                        comment.id
                    )),
                    url_hostname: Some("https://hacker-news.firebaseio.com".into()),
                    kids: comment.kids,
                })
            }
            HnItem::Job(job) => {
                let posted_at = datetime_from_hn_time(job.time);
                Ok(Self {
                    id: job.id,
                    posted_at,
                    posted_since: Self::formatted_posted_since(&posted_at),
                    by_username: job.by,
                    title: Some(job.title),
                    text: job.text,
                    score: job.score,
                    url: job.url.clone(),
                    url_hostname: job.url.map(|url| {
                        url::Url::parse(&url[..])
                            .map_err(HnCliError::UrlParsingError)
                            .expect("job URL parsing error") // TODO: avoid expect here
                            .host_str()
                            .expect("job URL must have an hostname")
                            .to_owned()
                    }),
                    kids: None,
                })
            }
            HnItem::Poll(poll) => {
                let posted_at = datetime_from_hn_time(poll.time);
                Ok(Self {
                    id: poll.id,
                    posted_at,
                    posted_since: Self::formatted_posted_since(&posted_at),
                    by_username: poll.by,
                    title: Some(poll.title),
                    text: None,
                    score: poll.score,
                    url: Some(format!(
                        "https://hacker-news.firebaseio.com/v0/item/{}.json?print=pretty",
                        poll.id
                    )),
                    url_hostname: Some("https://hacker-news.firebaseio.com".into()),
                    kids: poll.kids,
                })
            }
            _ => Err(HnCliError::HnItemProcessingError(
                value.get_id().to_string(),
            )),
        }
    }
}

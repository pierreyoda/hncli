//! See https://hn.algolia.com/api.

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::api::types::HnItemIdScalar;

#[derive(Clone, Debug, Deserialize)]
pub struct AlgoliaHnHits<H> {
    hits: Vec<H>,
}

impl<H> AlgoliaHnHits<H> {
    pub fn get_hits(&self) -> &[H] {
        &self.hits
    }
}

pub type AlgoliaHnStoriesHits = AlgoliaHnHits<AlgoliaHnStory>;
pub type AlgoliaHnCommentsHits = AlgoliaHnHits<AlgoliaHnComment>;

#[derive(Clone, Debug, Deserialize)]
pub struct AlgoliaHnStory {
    pub object_id: String,
    pub id: Option<HnItemIdScalar>,
    pub posted_at: DateTime<Utc>,
    pub title: String,
    pub url: String,
    pub author: String,
    pub text: Option<String>,
    pub points: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AlgoliaHnComment {
    pub object_id: String,
    pub parent_id: HnItemIdScalar,
    pub author: String,
    pub posted_at: DateTime<Utc>,
    pub story_id: HnItemIdScalar,
    pub story_url: String,
    pub text: String,
    pub points: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub enum AlgoliaHnHit {
    AlgoliaHnHitStory(AlgoliaHnStory),
    AlgoliaHnHitComment(AlgoliaHnComment),
}

pub trait AlgoliaHnFilter {
    fn to_query(&self) -> String;
}

/// One or multiple tag(s) can be applied to filter a full-text search in the Algolia Hacker News API.
///
/// Multiple tags at once have a AND behavior by default, but can be OR with parentheses around them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AlgoliaHnSearchTag {
    Story,
    Comment,
    Poll,
    PollOption,
    ShowHackerNews,
    AskHackerNews,
    FrontPage,
    AuthorUsername(String),
    StoryId(HnItemIdScalar),
}

impl AlgoliaHnFilter for AlgoliaHnSearchTag {
    fn to_query(&self) -> String {
        match self {
            Self::Story => "story".into(),
            Self::Comment => "comment".into(),
            Self::Poll => "poll".into(),
            Self::PollOption => "pollopt".into(),
            Self::ShowHackerNews => "show_hn".into(),
            Self::AskHackerNews => "ask_hn".into(),
            Self::FrontPage => "front_page".into(),
            Self::AuthorUsername(username) => format!("author_{}", username),
            Self::StoryId(story_id) => format!("story_{}", story_id),
        }
    }
}

/// Filter on a specific numerical condition (<, <=, =, > or >=).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AlgoliaHnNumericFilter {
    CreatedAt,
    Points,
    CommentsCount,
}

impl AlgoliaHnFilter for AlgoliaHnNumericFilter {
    fn to_query(&self) -> String {
        match self {
            Self::CreatedAt => "created_at_i".into(),
            Self::Points => "points".into(),
            Self::CommentsCount => "num_comments".into(),
        }
    }
}

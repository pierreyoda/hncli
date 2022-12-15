//! See https://hn.algolia.com/api.

use serde::Deserialize;

use crate::api::types::HnItemIdScalar;

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

#[derive(Debug, Deserialize)]
pub struct AlgoliaHnFullTextSearchHit {
    objectId: String,
}

impl AlgoliaHnFullTextSearchHit {
    pub fn try_parse_id(&self) -> Option<HnItemIdScalar> {
        self.objectId.parse::<HnItemIdScalar>().ok()
    }
}

#[derive(Debug, Deserialize)]
pub struct AlgoliaHnFullTextSearchResult {
    hits: Vec<AlgoliaHnFullTextSearchHit>,
    page: usize,
    query: String,
    /// Format: "query={...}".
    params: String,
}

impl AlgoliaHnFullTextSearchResult {
    pub fn get_hits(&self) -> &Vec<AlgoliaHnFullTextSearchHit> {
        &self.hits
    }
}

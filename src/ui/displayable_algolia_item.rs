use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use chrono::{DateTime, Utc};

use crate::api::types::HnItemIdScalar;

use super::utils::ItemWithId;

#[derive(Clone, Debug)]
pub struct DisplayableAlgoliaUser {
    username: String,
    about: Option<String>,
    karma: u32,
}

#[derive(Clone, Debug)]
pub struct DisplayableAlgoliaStory {
    pub id: Option<HnItemIdScalar>,
    pub posted_at: DateTime<Utc>,
    pub url: String,
    pub author: String,
    pub text: Option<String>,
    pub points: u32,
}

#[derive(Clone, Debug)]
pub struct DisplayableAlgoliaComment {
    pub object_id: HnItemIdScalar,
    pub parent_id: HnItemIdScalar,
    pub posted_at: DateTime<Utc>,
    pub story_id: HnItemIdScalar,
    pub story_url: String,
    pub text: String,
    pub points: u32,
}

/// A display-ready Hacker News Algolia item.
#[derive(Clone, Debug)]
pub enum DisplayableAlgoliaItem {
    User(DisplayableAlgoliaUser),
    Story(DisplayableAlgoliaStory),
    Comment(DisplayableAlgoliaComment),
}

impl ItemWithId<u64> for DisplayableAlgoliaItem {
    // TODO: make this less brittle
    fn get_id(&self) -> u64 {
        use DisplayableAlgoliaItem::*;

        let mut hasher = DefaultHasher::new();
        match self {
            User(user_data) => user_data.username.hash(&mut hasher),
            Story(story_data) => {
                format!("{}-{}", story_data.url, story_data.author).hash(&mut hasher)
            }
            Comment(comment_data) => comment_data.object_id.hash(&mut hasher),
        }
        hasher.finish()
    }
}

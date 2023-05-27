use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::api::{
    algolia_types::{AlgoliaHnComment, AlgoliaHnStory},
    types::HnItemIdScalar,
};

use super::utils::ItemWithId;

#[derive(Clone, Debug)]
pub struct DisplayableAlgoliaStory {
    pub object_id: String,
    pub id: Option<HnItemIdScalar>,
    pub title: String,
    pub url: Option<String>,
    pub author: String,
    pub text: Option<String>,
    pub points: u32,
}

impl From<AlgoliaHnStory> for DisplayableAlgoliaStory {
    fn from(value: AlgoliaHnStory) -> Self {
        Self {
            object_id: value.object_id,
            id: value.id,
            title: value.title,
            url: value.url,
            author: value.author,
            text: value.text,
            points: value.points,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DisplayableAlgoliaComment {
    pub object_id: String,
    pub parent_id: HnItemIdScalar,
    pub author: String,
    pub story_id: HnItemIdScalar,
    pub story_url: String,
    pub text: String,
    pub points: u32,
}

impl From<AlgoliaHnComment> for DisplayableAlgoliaComment {
    fn from(value: AlgoliaHnComment) -> Self {
        Self {
            object_id: value.object_id,
            parent_id: value.parent_id,
            author: value.author,
            story_id: value.story_id,
            story_url: value.story_url,
            text: value.comment_text,
            points: value.points.unwrap_or(0),
        }
    }
}

/// A display-ready Hacker News Algolia item.
#[derive(Clone, Debug)]
pub enum DisplayableAlgoliaItem {
    Story(DisplayableAlgoliaStory),
    Comment(DisplayableAlgoliaComment),
}

impl DisplayableAlgoliaItem {
    pub fn get_link(&self) -> Option<String> {
        use DisplayableAlgoliaItem::*;

        match self {
            Story(data) => data.url.clone(),
            Comment(_) => None,
        }
    }

    pub fn get_hacker_news_link(&self) -> String {
        use DisplayableAlgoliaItem::*;

        match self {
            Story(data) => format!("https://news.ycombinator.com/item?id={}", data.object_id),
            Comment(data) => format!("https://news.ycombinator.com/item?id={}", data.object_id),
        }
    }

    pub fn title(&self) -> &str {
        use DisplayableAlgoliaItem::*;

        match self {
            Story(data) => &data.title,
            Comment(data) => &data.author,
        }
    }

    pub fn meta(&self) -> String {
        use DisplayableAlgoliaItem::*;

        match self {
            Story(data) => format!("by {}, {} points", data.author, data.points),
            Comment(data) => format!("{} points", data.points),
        }
    }
}

impl ItemWithId<u64> for DisplayableAlgoliaItem {
    fn get_id(&self) -> u64 {
        use DisplayableAlgoliaItem::*;

        let mut hasher = DefaultHasher::new();
        match self {
            Story(story_data) => {
                format!("{:?}-{}", story_data.id, story_data.author).hash(&mut hasher)
            }
            Comment(comment_data) => comment_data.object_id.hash(&mut hasher),
        }
        hasher.finish()
    }
}

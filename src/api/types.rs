use serde::Deserialize;

pub type ItemIdScalar = u32;
pub type ItemDateScalar = u64;

/// A `Story` in the HackerNews API.
///
/// # Example
///
/// ```json
/// {
///   "by" : "dhouston",
///   "descendants" : 71,
///   "id" : 8863,
///   "kids" : [ 8952, 9224, 8917, 8884, 8887, 8943, 8869, 8958, 9005, 9671, 8940, 9067, 8908, 9055, 8865, 8881, 8872, 8873, 8955, 10403, 8903, 8928, 9125, 8998, 8901, 8902, 8907, 8894, 8878, 8870, 8980, 8934, 8876 ],
///   "score" : 111,
///   "time" : 1175714200,
///   "title" : "My YC app: Dropbox - Throw away your USB drive",
///   "type" : "story",
///   "url" : "http://www.getdropbox.com/u/2/screencast.html"
/// }
/// ```
#[derive(Debug, Deserialize)]
pub struct HnStory {
    /// Unique ID of this Item.
    id: ItemIdScalar,
    /// Unix timestamp for the creation time.
    time: ItemDateScalar,
    /// Username of the story's author.
    by: String,
    /// Score of the story.
    score: u32,
    /// Title of the story.
    title: String,
    /// *HTML* text of the story, if any.
    text: Option<String>,
    /// URL of the story, if any.
    url: Option<String>,
    /// Total number of comments on the story.
    descendants: u32,
    /// IDs of the comments on the story, if any, in ranked display order.
    kids: Option<Vec<u32>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_story_parsing() {
        let json = r#"{
            "by" : "dhouston",
            "descendants" : 71,
            "id" : 8863,
            "kids" : [ 8952, 9224, 8917, 8884, 8887, 8943, 8869, 8958, 9005, 9671, 8940, 9067, 8908, 9055, 8865, 8881, 8872, 8873, 8955, 10403, 8903, 8928, 9125, 8998, 8901, 8902, 8907, 8894, 8878, 8870, 8980, 8934, 8876 ],
            "score" : 111,
            "time" : 1175714200,
            "title" : "My YC app: Dropbox - Throw away your USB drive",
            "type" : "story",
            "url" : "http://www.getdropbox.com/u/2/screencast.html"
        }"#;

        let story: HnStory = serde_json::from_str(&json).unwrap();
        assert_eq!(story.id, 8863);
        assert_eq!(story.time, 1175714200);
        assert_eq!(story.by, "dhouston".to_string());
        assert_eq!(story.score, 111);
        assert_eq!(
            story.title,
            "My YC app: Dropbox - Throw away your USB drive".to_string()
        );
        assert_eq!(story.text, None);
        assert_eq!(
            story.url,
            Some("http://www.getdropbox.com/u/2/screencast.html".to_string())
        );
        assert_eq!(story.descendants, 71);
        assert_eq!(
            story.kids,
            Some(vec![
                8952, 9224, 8917, 8884, 8887, 8943, 8869, 8958, 9005, 9671, 8940, 9067, 8908, 9055,
                8865, 8881, 8872, 8873, 8955, 10403, 8903, 8928, 9125, 8998, 8901, 8902, 8907,
                8894, 8878, 8870, 8980, 8934, 8876
            ])
        );
    }
}

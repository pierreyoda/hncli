//! See https://github.com/HackerNews/API.

use serde::Deserialize;

pub type HnItemIdScalar = u32;
pub type HnItemDateScalar = u64;

/// An `Item` in the HackerNews API covers everything except `User`s.
///
/// Used for deserialization, with dispatch on `type`.
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum HnItem {
    Story(HnStory),
    Comment(HnComment),
    Job(HnJob),
    Poll(HnPoll),
    PollOpt(HnPollOption),
    #[serde(other)]
    Null,
}

impl HnItem {
    pub fn is_null(&self) -> bool {
        matches!(self, HnItem::Null)
    }

    /// Get the ID of the item.
    pub fn get_id(&self) -> HnItemIdScalar {
        use HnItem::*;

        match self {
            Null => 0,
            Story(story) => story.id,
            Comment(comment) => comment.id,
            Job(job) => job.id,
            Poll(poll) => poll.id,
            PollOpt(poll_option) => poll_option.id,
        }
    }

    /// Get the `kids`, if any, of the item.
    pub fn get_kids(&self) -> Option<&[HnItemIdScalar]> {
        use HnItem::*;

        match self {
            Null => None,
            Story(story) => story.kids.as_deref(),
            Comment(comment) => comment.kids.as_deref(),
            Job(_) => None,
            Poll(poll) => poll.kids.as_deref(),
            PollOpt(_) => None,
        }
    }
}

/// A `Story` in the HackerNews API.
///
/// # Example
///
/// ```json
/// {
///   "by" : "dhouston",
///   "descendants" : 71,
///   "id" : 8863,
///   "kids" : [ 8952, 9224, 8917, 8884, 8887, 8943, 8869, 8958, /** ... */ ],
///   "score" : 111,
///   "time" : 1175714200,
///   "title" : "My YC app: Dropbox - Throw away your USB drive",
///   "type" : "story",
///   "url" : "http://www.getdropbox.com/u/2/screencast.html"
/// }
/// ```
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct HnStory {
    /// Unique ID of this Item.
    pub id: HnItemIdScalar,
    /// Unix timestamp for the creation time.
    pub time: HnItemDateScalar,
    /// Username of the story's author.
    pub by: String,
    /// Score of the story.
    pub score: u32,
    /// Title of the story.
    pub title: String,
    /// *HTML* text of the story, if any.
    pub text: Option<String>,
    /// URL of the story, if any.
    pub url: Option<String>,
    /// Total number of comments on the story.
    pub descendants: u32,
    /// IDs of the comments on the story, if any, in ranked display order.
    pub kids: Option<Vec<HnItemIdScalar>>,
}

/// A `Comment` in the HackerNews API.
///
/// # Example
///
/// ```json
/// {
///    "by" : "norvig",
///    "id" : 2921983,
///    "kids" : [ 2922097, 2922429, 2924562, 2922709, 2922573, 2922140, 2922141 ],
///    "parent" : 2921506,
///    "text" : "Aw shucks, guys ... you make me blush with your compliments.<p>Tell you what, [...]",
///    "time" : 1314211127,
///    "type" : "comment"
/// }
/// ```
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct HnComment {
    /// Unique ID of this Item.
    pub id: HnItemIdScalar,
    /// Unix timestamp for the creation time.
    pub time: HnItemDateScalar,
    /// Username of the comment's author.
    pub by: String,
    /// Score of the comment, if defined.
    pub score: Option<u32>,
    /// The comment's parent, either a story or another comment.
    pub parent: HnItemIdScalar,
    /// The IDs of the comment's sub-comments, in ranked display order.
    pub kids: Option<Vec<HnItemIdScalar>>,
    /// *HTML* text of the comment.
    pub text: String,
}

/// A `Job` posting in the HackerNews API.
///
/// # Example
///
/// ```json
/// {
///   "by" : "justin",
///   "id" : 192327,
///   "score" : 6,
///   "text" : "Justin.tv is the biggest live video site online. [...]",
///   "time" : 1210981217,
///   "title" : "Justin.tv is looking for a Lead Flash Engineer!",
///   "type" : "job",
///   "url" : ""
/// }
/// ```
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct HnJob {
    /// Unique ID of this Item.
    pub id: HnItemIdScalar,
    /// Unix timestamp for the creation time.
    pub time: HnItemDateScalar,
    /// Username of the job's author.
    pub by: String,
    /// Score of the job.
    pub score: u32,
    /// Title of the job.
    pub title: String,
    /// *HTML* text of the job, if any.
    pub text: Option<String>,
    /// URL of the job, if any.
    pub url: Option<String>,
}

/// A `Poll` in the HackerNews API.
///
/// # Example
///
/// ```json
/// {
///   "by" : "pg",
///   "descendants" : 54,
///   "id" : 126809,
///   "kids" : [ 126822, 126823, 126993, 126824, 126934, 127411, 126888, 127681, 126818, 126816, 126854, 127095, 126861, 127313, 127299, 126859, 126852, 126882, 126832, 127072, 127217, 126889, 127535, 126917, 126875 ],
///   "parts" : [ 126810, 126811, 126812 ],
///   "score" : 46,
///   "text" : "",
///   "time" : 1204403652,
///   "title" : "Poll: What would happen if News.YC had explicit support for polls?",
///   "type" : "poll"
/// }
/// ```
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct HnPoll {
    /// Unique ID of this Item.
    pub id: HnItemIdScalar,
    /// Unix timestamp for the creation time.
    pub time: HnItemDateScalar,
    /// Username of the poll's author.
    pub by: String,
    /// Score of the poll.
    pub score: u32,
    /// Title of the poll.
    pub title: String,
    /// Options of the poll.
    pub parts: Vec<HnItemIdScalar>,
    /// Total number of comments on the poll.
    pub descendants: u32,
    /// IDs of the comments on the poll, if any, in ranked display order.
    pub kids: Option<Vec<HnItemIdScalar>>,
}

/// A Poll Option in the HackerNews API.
///
/// # Example
///
/// {
///   "by" : "pg",
///   "id" : 160705,
///   "poll" : 160704,
///   "score" : 335,
///   "text" : "Yes, ban them; I'm tired of seeing Valleywag stories on News.YC.",
///   "time" : 1207886576,
///   "type" : "pollopt"
/// }
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct HnPollOption {
    /// Unique ID of this Item.
    pub id: HnItemIdScalar,
    /// Unix timestamp for the creation time.
    pub time: HnItemDateScalar,
    /// Username of the poll's author.
    pub by: String,
    /// Score of the poll option.
    pub score: u32,
    /// Text of the poll option.
    pub text: String,
}

/// A `User` in the HackerNews API.
///
/// # Example
///
/// ```json
/// {
///    "about" : "This is a test",
///    "created" : 1173923446,
///    "delay" : 0,
///    "id" : "jl",
///    "karma" : 2937,
///    "submitted" : [ 8265435, 8168423, 8090946, /** ... */ ]
/// }
/// ```
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct HnUser {
    /// **Case-sensitive**, unique username.
    pub id: String,
    /// Unix timestamp for the user's registration date.
    pub created: HnItemDateScalar,
    /// Total karma of the user.
    pub karma: u32,
    /// *HTML* description of the user, if any.
    pub about: Option<String>,
    /// IDs of the user's submitted items.
    pub submitted: Vec<HnItemIdScalar>,
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

        let parsed: HnStory = serde_json::from_str(&json).unwrap();
        let expected = HnStory {
            id: 8863,
            time: 1175714200,
            by: "dhouston".into(),
            score: 111,
            title: "My YC app: Dropbox - Throw away your USB drive".into(),
            text: None,
            url: Some("http://www.getdropbox.com/u/2/screencast.html".into()),
            descendants: 71,
            kids: Some(vec![
                8952, 9224, 8917, 8884, 8887, 8943, 8869, 8958, 9005, 9671, 8940, 9067, 8908, 9055,
                8865, 8881, 8872, 8873, 8955, 10403, 8903, 8928, 9125, 8998, 8901, 8902, 8907,
                8894, 8878, 8870, 8980, 8934, 8876,
            ]),
        };

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_comment_parsing() {
        let json = r#"{
            "by" : "norvig",
            "id" : 2921983,
            "kids" : [ 2922097, 2922429, 2924562, 2922709, 2922573, 2922140, 2922141 ],
            "parent" : 2921506,
            "text" : "Aw shucks, guys ... you make me blush with your compliments.<p>Tell you what, Ill make a deal: I'll keep writing if you keep reading. K?",
            "time" : 1314211127,
            "type" : "comment"
        }"#;

        let parsed: HnComment = serde_json::from_str(json).unwrap();
        let expected = HnComment {
            id : 2921983,
            time : 1314211127,
            by : "norvig".into(),
            score: None,
            parent : 2921506,
            kids : Some(vec![2922097, 2922429, 2924562, 2922709, 2922573, 2922140, 2922141]),
            text : "Aw shucks, guys ... you make me blush with your compliments.<p>Tell you what, Ill make a deal: I'll keep writing if you keep reading. K?".into(),
        };

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_job_parsing() {
        let json = r#"{
            "by" : "justin",
            "id" : 192327,
            "score" : 6,
            "text" : "Justin.tv is the biggest live video site online. We serve hundreds of thousands of video streams a day, and have supported up to 50k live concurrent viewers. Our site is growing every week, and we just added a 10 gbps line to our colo. Our unique visitors are up 900% since January.<p>There are a lot of pieces that fit together to make Justin.tv work: our video cluster, IRC server, our web app, and our monitoring and search services, to name a few. A lot of our website is dependent on Flash, and we're looking for talented Flash Engineers who know AS2 and AS3 very well who want to be leaders in the development of our Flash.<p>Responsibilities<p><pre><code>    * Contribute to product design and implementation discussions\n    * Implement projects from the idea phase to production\n    * Test and iterate code before and after production release \n</code></pre>\nQualifications<p><pre><code>    * You should know AS2, AS3, and maybe a little be of Flex.\n    * Experience building web applications.\n    * A strong desire to work on website with passionate users and ideas for how to improve it.\n    * Experience hacking video streams, python, Twisted or rails all a plus.\n</code></pre>\nWhile we're growing rapidly, Justin.tv is still a small, technology focused company, built by hackers for hackers. Seven of our ten person team are engineers or designers. We believe in rapid development, and push out new code releases every week. We're based in a beautiful office in the SOMA district of SF, one block from the caltrain station. If you want a fun job hacking on code that will touch a lot of people, JTV is for you.<p>Note: You must be physically present in SF to work for JTV. Completing the technical problem at <a href=\"http://www.justin.tv/problems/bml\" rel=\"nofollow\">http://www.justin.tv/problems/bml</a> will go a long way with us. Cheers!",
            "time" : 1210981217,
            "title" : "Justin.tv is looking for a Lead Flash Engineer!",
            "type" : "job",
            "url" : ""
        }"#;

        let parsed: HnJob = serde_json::from_str(json).unwrap();
        let expected = HnJob {
            id: 192327,
            time: 1210981217,
            by: "justin".into(),
            score: 6,
            title: "Justin.tv is looking for a Lead Flash Engineer!".into(),
            text: Some("Justin.tv is the biggest live video site online. We serve hundreds of thousands of video streams a day, and have supported up to 50k live concurrent viewers. Our site is growing every week, and we just added a 10 gbps line to our colo. Our unique visitors are up 900% since January.<p>There are a lot of pieces that fit together to make Justin.tv work: our video cluster, IRC server, our web app, and our monitoring and search services, to name a few. A lot of our website is dependent on Flash, and we're looking for talented Flash Engineers who know AS2 and AS3 very well who want to be leaders in the development of our Flash.<p>Responsibilities<p><pre><code>    * Contribute to product design and implementation discussions\n    * Implement projects from the idea phase to production\n    * Test and iterate code before and after production release \n</code></pre>\nQualifications<p><pre><code>    * You should know AS2, AS3, and maybe a little be of Flex.\n    * Experience building web applications.\n    * A strong desire to work on website with passionate users and ideas for how to improve it.\n    * Experience hacking video streams, python, Twisted or rails all a plus.\n</code></pre>\nWhile we're growing rapidly, Justin.tv is still a small, technology focused company, built by hackers for hackers. Seven of our ten person team are engineers or designers. We believe in rapid development, and push out new code releases every week. We're based in a beautiful office in the SOMA district of SF, one block from the caltrain station. If you want a fun job hacking on code that will touch a lot of people, JTV is for you.<p>Note: You must be physically present in SF to work for JTV. Completing the technical problem at <a href=\"http://www.justin.tv/problems/bml\" rel=\"nofollow\">http://www.justin.tv/problems/bml</a> will go a long way with us. Cheers!".into()),
            url: Some("".into())
        };

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_user_parsing() {
        let json = r#"{
            "about" : "This is a test",
            "created" : 1173923446,
            "delay" : 0,
            "id" : "jl",
            "karma" : 2937,
            "submitted" : [ 8265435, 8168423, 8090946, 8090326, 7699907, 7637962, 7596179, 7596163, 7594569, 7562135, 7562111, 7494708, 7494171, 7488093, 7444860, 7327817, 7280290, 7278694, 7097557, 7097546, 7097254, 7052857, 7039484, 6987273, 6649999, 6649706, 6629560, 6609127, 6327951, 6225810, 6111999, 5580079, 5112008, 4907948, 4901821, 4700469, 4678919, 3779193, 3711380, 3701405, 3627981, 3473004, 3473000, 3457006, 3422158, 3136701, 2943046, 2794646, 2482737, 2425640, 2411925, 2408077, 2407992, 2407940, 2278689, 2220295, 2144918, 2144852, 1875323, 1875295, 1857397, 1839737, 1809010, 1788048, 1780681, 1721745, 1676227, 1654023, 1651449, 1641019, 1631985, 1618759, 1522978, 1499641, 1441290, 1440993, 1436440, 1430510, 1430208, 1385525, 1384917, 1370453, 1346118, 1309968, 1305415, 1305037, 1276771, 1270981, 1233287, 1211456, 1210688, 1210682, 1194189, 1193914, 1191653, 1190766, 1190319, 1189925, 1188455, 1188177, 1185884, 1165649, 1164314, 1160048, 1159156, 1158865, 1150900, 1115326, 933897, 924482, 923918, 922804, 922280, 922168, 920332, 919803, 917871, 912867, 910426, 902506, 891171, 807902, 806254, 796618, 786286, 764412, 764325, 642566, 642564, 587821, 575744, 547504, 532055, 521067, 492164, 491979, 383935, 383933, 383930, 383927, 375462, 263479, 258389, 250751, 245140, 243472, 237445, 229393, 226797, 225536, 225483, 225426, 221084, 213940, 213342, 211238, 210099, 210007, 209913, 209908, 209904, 209903, 170904, 165850, 161566, 158388, 158305, 158294, 156235, 151097, 148566, 146948, 136968, 134656, 133455, 129765, 126740, 122101, 122100, 120867, 120492, 115999, 114492, 114304, 111730, 110980, 110451, 108420, 107165, 105150, 104735, 103188, 103187, 99902, 99282, 99122, 98972, 98417, 98416, 98231, 96007, 96005, 95623, 95487, 95475, 95471, 95467, 95326, 95322, 94952, 94681, 94679, 94678, 94420, 94419, 94393, 94149, 94008, 93490, 93489, 92944, 92247, 91713, 90162, 90091, 89844, 89678, 89498, 86953, 86109, 85244, 85195, 85194, 85193, 85192, 84955, 84629, 83902, 82918, 76393, 68677, 61565, 60542, 47745, 47744, 41098, 39153, 38678, 37741, 33469, 12897, 6746, 5252, 4752, 4586, 4289 ]
        }"#;

        let parsed: HnUser = serde_json::from_str(json).unwrap();
        let expected = HnUser {
            id: "jl".into(),
            created: 1173923446,
            karma: 2937,
            about: Some("This is a test".into()),
            submitted: vec![
                8265435, 8168423, 8090946, 8090326, 7699907, 7637962, 7596179, 7596163, 7594569,
                7562135, 7562111, 7494708, 7494171, 7488093, 7444860, 7327817, 7280290, 7278694,
                7097557, 7097546, 7097254, 7052857, 7039484, 6987273, 6649999, 6649706, 6629560,
                6609127, 6327951, 6225810, 6111999, 5580079, 5112008, 4907948, 4901821, 4700469,
                4678919, 3779193, 3711380, 3701405, 3627981, 3473004, 3473000, 3457006, 3422158,
                3136701, 2943046, 2794646, 2482737, 2425640, 2411925, 2408077, 2407992, 2407940,
                2278689, 2220295, 2144918, 2144852, 1875323, 1875295, 1857397, 1839737, 1809010,
                1788048, 1780681, 1721745, 1676227, 1654023, 1651449, 1641019, 1631985, 1618759,
                1522978, 1499641, 1441290, 1440993, 1436440, 1430510, 1430208, 1385525, 1384917,
                1370453, 1346118, 1309968, 1305415, 1305037, 1276771, 1270981, 1233287, 1211456,
                1210688, 1210682, 1194189, 1193914, 1191653, 1190766, 1190319, 1189925, 1188455,
                1188177, 1185884, 1165649, 1164314, 1160048, 1159156, 1158865, 1150900, 1115326,
                933897, 924482, 923918, 922804, 922280, 922168, 920332, 919803, 917871, 912867,
                910426, 902506, 891171, 807902, 806254, 796618, 786286, 764412, 764325, 642566,
                642564, 587821, 575744, 547504, 532055, 521067, 492164, 491979, 383935, 383933,
                383930, 383927, 375462, 263479, 258389, 250751, 245140, 243472, 237445, 229393,
                226797, 225536, 225483, 225426, 221084, 213940, 213342, 211238, 210099, 210007,
                209913, 209908, 209904, 209903, 170904, 165850, 161566, 158388, 158305, 158294,
                156235, 151097, 148566, 146948, 136968, 134656, 133455, 129765, 126740, 122101,
                122100, 120867, 120492, 115999, 114492, 114304, 111730, 110980, 110451, 108420,
                107165, 105150, 104735, 103188, 103187, 99902, 99282, 99122, 98972, 98417, 98416,
                98231, 96007, 96005, 95623, 95487, 95475, 95471, 95467, 95326, 95322, 94952, 94681,
                94679, 94678, 94420, 94419, 94393, 94149, 94008, 93490, 93489, 92944, 92247, 91713,
                90162, 90091, 89844, 89678, 89498, 86953, 86109, 85244, 85195, 85194, 85193, 85192,
                84955, 84629, 83902, 82918, 76393, 68677, 61565, 60542, 47745, 47744, 41098, 39153,
                38678, 37741, 33469, 12897, 6746, 5252, 4752, 4586, 4289,
            ],
        };

        assert_eq!(parsed, expected);
    }
}

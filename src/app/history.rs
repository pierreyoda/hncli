use std::{
    collections::HashMap,
    fs::{create_dir_all, read_to_string, write},
    path::PathBuf,
};

use chrono::{serde::ts_seconds, DateTime, Utc};
use log::warn;
use serde::{Deserialize, Serialize};

use crate::{
    api::types::HnItemIdScalar,
    config::get_project_os_directory,
    errors::{HnCliError, Result},
};

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TopLevelCommentHistoryData {
    #[serde(with = "ts_seconds")]
    datetime: DateTime<Utc>,
    top_level_comment_id: HnItemIdScalar,
}

/// TODO: support more than top-level comments (would also need refactoring elsewhere)
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum SynchronizedHistoryItem {
    /// Saves the navigation state of a top-level comment for a given Item thread.
    TopLevelComment(TopLevelCommentHistoryData),
}

impl SynchronizedHistoryItem {
    /// The datetime at which this `SynchronizedHistoryItem` was first inserted or last updated.
    fn get_timestamp(&self) -> &DateTime<Utc> {
        match self {
            Self::TopLevelComment(data) => &data.datetime,
        }
    }

    /// Get the stored ID corresponding to the saved navigation state.
    fn get_value(&self) -> HnItemIdScalar {
        match self {
            Self::TopLevelComment(data) => data.top_level_comment_id,
        }
    }

    /// Update the stored ID corresponding to the saved navigation state.
    fn set_value(&mut self, id: HnItemIdScalar) {
        match self {
            Self::TopLevelComment(ref mut data) => {
                data.datetime = Utc::now();
                data.top_level_comment_id = id;
            }
        }
    }
}

type SynchronizedHistoryItemStorage = HashMap<HnItemIdScalar, SynchronizedHistoryItem>;

#[derive(Debug, Deserialize, Serialize)]
pub struct SynchronizedHistory {
    /// Stores the latest focused top-level comment for a given Hacker News item.
    ///
    /// Also keeps track of the insertion datetime to enforce hard limits on the history size.
    latest_top_level_comments_per_item_map: SynchronizedHistoryItemStorage,
}

impl SynchronizedHistory {
    fn empty() -> Self {
        Self {
            latest_top_level_comments_per_item_map: SynchronizedHistoryItemStorage::with_capacity(
                SYNCHRONIZED_HISTORY_ITEMS_LIMIT,
            ),
        }
    }

    /// Instantiate the synchronized history from the given JSON file.
    fn read_from_json_file(history_filepath: PathBuf) -> Self {
        // TODO: maybe a simple macro to reduce Result handling boilerplate
        // File existence/permissions check
        match history_filepath.try_exists().map_err(|err| {
            HnCliError::HistorySynchronizationError(format!(
                "cannot check if history file ({}) exists: {}",
                history_filepath.display(),
                err
            ))
        }) {
            Err(why) => {
                warn!("{}", why);
                return Self::empty();
            }
            Ok(exists) => {
                if !exists {
                    warn!(
                        "history file ({}) does not exist yet",
                        history_filepath.display()
                    );
                    return Self::empty();
                }
            }
        }

        // Read
        let history_raw = match read_to_string(&history_filepath).map_err(|err| {
            HnCliError::HistorySynchronizationError(format!(
                "cannot open history file ({}): {}",
                history_filepath.display(),
                err
            ))
        }) {
            Ok(raw) => raw,
            Err(why) => {
                warn!("{}", why);
                return Self::empty();
            }
        };

        // Deserialize
        let synchronized_history: Self = match serde_json::from_str(&history_raw).map_err(|err| {
            HnCliError::HistorySynchronizationError(format!("cannot deserialize history: {}", err))
        }) {
            Ok(read_history) => read_history,
            Err(why) => {
                warn!("{}", why);
                return Self::empty();
            }
        };

        synchronized_history
    }

    /// Write the synchronized history to the given JSON file (erasing it if needed).
    ///
    /// This method should be not be called at every app interaction possible,
    /// for instance not at every top-level focused comment change.
    ///
    /// TODO: there is an edge case where, after heavy continuous usage, the storage(s) can run out of memory. Find a solution applied before saving.
    fn write_to_json_file(&self, history_filepath: PathBuf) -> Result<()> {
        let history_directory = history_filepath.parent().expect(
            "SynchronizedHistory.write_to_json_file: history filepath parent folder can be read",
        );
        create_dir_all(history_directory).map_err(|err| {
            HnCliError::HistorySynchronizationError(format!(
                "cannot create history directory ({:?}): {}",
                history_directory.display(),
                err
            ))
        })?;

        let limited_latest_top_level_comments_per_item_map = Self::enforced_history_limit(
            &self.latest_top_level_comments_per_item_map,
            SYNCHRONIZED_HISTORY_ITEMS_LIMIT,
        );
        let limited_synchronized_history = Self {
            latest_top_level_comments_per_item_map: limited_latest_top_level_comments_per_item_map,
        };

        let history_raw = serde_json::to_string(&limited_synchronized_history).map_err(|err| {
            HnCliError::HistorySynchronizationError(format!("cannot serialize history: {}", err))
        })?;

        write(&history_filepath, history_raw).map_err(|err| {
            HnCliError::HistorySynchronizationError(format!(
                "cannot save history file ({:?}): {}",
                history_filepath.display(),
                err
            ))
        })
    }

    /// Enforce an arbitrary items count limit on the stored navigation data.
    fn enforced_history_limit(
        storage: &SynchronizedHistoryItemStorage,
        limit: usize,
    ) -> SynchronizedHistoryItemStorage {
        let mut storage_entries: Vec<_> = storage.iter().collect();
        storage_entries.sort_by(|(_id_a, item_a), (_id_b, item_b)| {
            item_a.get_timestamp().partial_cmp(item_b.get_timestamp()).expect("SynchronizedHistory::enforced_history_limit must be able to compare two Utc timestamps.")
        });
        let mut limited_storage = SynchronizedHistoryItemStorage::with_capacity(limit);
        for (id, storage_item) in storage_entries.iter().rev().take(limit) {
            let limited_storage_item: SynchronizedHistoryItem = (*storage_item).clone();
            limited_storage.insert(**id, limited_storage_item);
        }
        limited_storage
    }
}

/// Maximum number of entries that will be kept in the history file.
pub const SYNCHRONIZED_HISTORY_ITEMS_LIMIT: usize = 500;

/// Responsible for restoring navigation state in the application from previous sessions.
#[derive(Debug)]
pub struct AppHistory {
    /// File-synchronized part of the navigation History.
    ///
    /// Reading must be done at application startup, and writing as rarely as possible.
    synchronized: SynchronizedHistory,
}

impl AppHistory {
    pub fn restored() -> Self {
        match Self::get_history_file_path() {
            Ok(history_file_path) => Self {
                synchronized: SynchronizedHistory::read_from_json_file(history_file_path),
            },
            Err(why) => {
                warn!(
                    "History: cannot retrieve OS filepath for history.json (reading history): {}",
                    why
                );
                Self {
                    synchronized: SynchronizedHistory::empty(),
                }
            }
        }
    }

    /// Persist the history in OS-dependent JSON storage.
    ///
    /// Should not be called too often for performance reasons.
    pub fn persist(&self) {
        match Self::get_history_file_path() {
            Ok(history_filepath) => match self.synchronized.write_to_json_file(history_filepath) {
                Ok(()) => (),
                Err(why) => {
                    warn!("History.persist error: {}", why);
                }
            },
            Err(why) => {
                warn!(
                    "History: cannot retrieve OS filepath for history.json (writing history): {}",
                    why
                );
            }
        }
    }

    pub fn persist_top_level_comment_id_for_story(
        &mut self,
        story_id: HnItemIdScalar,
        top_level_comment_id: HnItemIdScalar,
    ) {
        if let Some(entry) = self
            .synchronized
            .latest_top_level_comments_per_item_map
            .get_mut(&story_id)
        {
            entry.set_value(top_level_comment_id);
        } else {
            self.synchronized
                .latest_top_level_comments_per_item_map
                .insert(
                    story_id,
                    SynchronizedHistoryItem::TopLevelComment(TopLevelCommentHistoryData {
                        top_level_comment_id,
                        datetime: Utc::now(),
                    }),
                );
        };
    }

    pub fn restored_top_level_comment_id_for_story(
        &self,
        story_id: HnItemIdScalar,
    ) -> Option<HnItemIdScalar> {
        self.synchronized
            .latest_top_level_comments_per_item_map
            .get(&story_id)
            .map(|entry| entry.get_value())
    }

    fn get_history_file_path() -> Result<PathBuf> {
        let project_os_directory = get_project_os_directory()?;
        Ok(project_os_directory.join("history.json"))
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::*;

    #[test]
    fn test_simple_item_top_level_persist_comment_id_scenario() {
        let mut history = AppHistory {
            synchronized: SynchronizedHistory::empty(),
        };

        // viewed item 1 and left while focused on comment ID 123
        history.persist_top_level_comment_id_for_story(1, 123);

        // viewed item 2 and left while focused on comment ID 456
        history.persist_top_level_comment_id_for_story(2, 456);

        // viewed item 1 again, left while focused on comment ID 1230
        history.persist_top_level_comment_id_for_story(1, 1230);

        // basic assertions
        assert_eq!(
            history.restored_top_level_comment_id_for_story(1),
            Some(1230)
        );
        assert_eq!(
            history.restored_top_level_comment_id_for_story(2),
            Some(456)
        );
        assert_eq!(history.restored_top_level_comment_id_for_story(3), None);
    }

    #[test]
    fn test_history_storage_limit_enforcing() {
        let mut storage = SynchronizedHistoryItemStorage::new();
        storage.insert(
            456,
            SynchronizedHistoryItem::TopLevelComment(TopLevelCommentHistoryData {
                datetime: Utc::now(),
                top_level_comment_id: 4567,
            }),
        );
        storage.insert(
            123,
            SynchronizedHistoryItem::TopLevelComment(TopLevelCommentHistoryData {
                datetime: Utc::now() - Duration::minutes(3),
                top_level_comment_id: 1231,
            }),
        );
        storage.insert(
            789,
            SynchronizedHistoryItem::TopLevelComment(TopLevelCommentHistoryData {
                datetime: Utc::now() + Duration::seconds(37),
                top_level_comment_id: 7895,
            }),
        );

        let limited_storage = SynchronizedHistory::enforced_history_limit(&storage, 2);

        assert_eq!(limited_storage.len(), 2);
        assert!(!limited_storage.contains_key(&123));
        assert!(limited_storage.contains_key(&456));
        assert!(limited_storage.contains_key(&789));
    }
}

use chrono::{DateTime, NaiveDateTime, Utc};
use num_traits::Num;
use tui::widgets::ListState;

use crate::api::types::HnItemDateScalar;

pub trait ItemWithId<N: Copy + Num + Ord> {
    fn get_id(&self) -> N;
}

/// Wrapper around a tui `ListState` to provide wrap-around navigation.
#[derive(Debug)]
pub struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    /// Create a new `StatefulList<T>` with the given items.
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        Self {
            state: ListState::default(),
            items,
        }
    }

    /// Replace the current items with the given ones.
    pub fn replace_items(&mut self, items: Vec<T>) {
        self.items = items;
        // TODO: simple reconciliation algorithm
        self.unselect();
    }

    /// Select the next item, starting at 0 if none is selected or
    /// wrapping around to 0 if at the end of the list.
    pub fn next(&mut self) {
        self.state.select(Some(match self.state.selected() {
            None => 0,
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
        }));
    }

    /// Select the previous item, starting at 0 if none is selected or
    /// wrapping around to `items.len() - 1` if at the start of the list.
    pub fn previous(&mut self) {
        self.state.select(Some(match self.state.selected() {
            None => 0,
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
        }))
    }

    /// Clear the current selection.
    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    /// Get the current `ListState`.
    pub fn get_state(&self) -> &ListState {
        &self.state
    }
}

/// Creates a chrono `DateTime` from a Hacker News Unix timestamp.
pub fn datetime_from_hn_time(time: HnItemDateScalar) -> DateTime<Utc> {
    let timestamp = time as i64;
    let naive = NaiveDateTime::from_timestamp(timestamp, 0);
    DateTime::from_utc(naive, Utc)
}

#[cfg(test)]
mod tests {
    use super::{datetime_from_hn_time, StatefulList};

    #[test]
    pub fn test_stateful_list_wrapper() {
        let mut stateful_list = StatefulList::with_items(vec![0, 1]);
        assert_eq!(stateful_list.get_state().selected(), None);

        stateful_list.next();
        assert_eq!(stateful_list.get_state().selected(), Some(0));
        stateful_list.next();
        assert_eq!(stateful_list.get_state().selected(), Some(1));
        stateful_list.next();
        assert_eq!(stateful_list.get_state().selected(), Some(0));

        stateful_list.previous();
        assert_eq!(stateful_list.get_state().selected(), Some(1));
        stateful_list.previous();
        assert_eq!(stateful_list.get_state().selected(), Some(0));

        stateful_list.unselect();
        assert_eq!(stateful_list.get_state().selected(), None);
        stateful_list.previous();
        assert_eq!(stateful_list.get_state().selected(), Some(0));
    }

    #[test]
    pub fn test_datetime_from_hn_time() {
        let date = datetime_from_hn_time(1203647620);
        let formatted_date = date.format("%Y-%m-%d %H:%M:%S").to_string();

        assert_eq!(formatted_date, "2008-02-22 02:33:40".to_string());
    }
}

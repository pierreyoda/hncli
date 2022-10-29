use chrono::{DateTime, NaiveDateTime, Utc};
use num_traits::Num;
use tui::widgets::ListState;

use crate::api::types::HnItemDateScalar;

pub trait ItemWithId<N: Copy + Num + Ord> {
    fn get_id(&self) -> N;
}

/// Wrapper around a tui `ListState` to provide wrap-around navigation.
#[derive(Debug)]
pub struct StatefulList<N, T>
where
    N: Copy + Num + Ord + Default,
    T: Clone + ItemWithId<N>,
{
    state: ListState,
    items: Vec<T>,
    // NB: this field is only there to prevent the "N parameter not used" compilation error
    _n: N,
}

impl<N, T> StatefulList<N, T>
where
    N: Copy + Num + Ord + Default,
    T: Clone + ItemWithId<N>,
{
    /// Create a new `StatefulList<T>` with the given items.
    pub fn with_items(items: Vec<T>) -> StatefulList<N, T> {
        Self {
            state: ListState::default(),
            items,
            _n: Default::default(),
        }
    }

    /// Get the stored items.
    pub fn get_items(&self) -> &Vec<T> {
        &self.items
    }

    /// Replace the current items with the given ones.
    pub fn replace_items(&mut self, items: Vec<T>) {
        let old_items = self.items.clone();
        self.items = items;
        self.reconciliate_current_selection(old_items);
    }

    /// Select the next item, starting at 0 if none is selected or
    /// wrapping around to 0 if at the end of the list.
    pub fn next(&mut self) {
        self.state.select(Some(match self.state.selected() {
            None => 0,
            Some(i) => (i + 1) % self.items.len(),
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
    pub fn get_state(&mut self) -> &mut ListState {
        &mut self.state
    }

    fn reconciliate_current_selection(&mut self, old_items: Vec<T>) {
        let mut found = false;
        if let Some(selected_index) = self.state.selected() {
            if let Some(old_selected_item) = old_items.get(selected_index) {
                let old_selected_item_id = old_selected_item.get_id();
                if let Some(new_selected_index) = self
                    .items
                    .iter()
                    .position(|i| i.get_id() == old_selected_item_id)
                {
                    found = true;
                    self.state.select(Some(new_selected_index));
                }
            }
        }
        if !found {
            self.unselect();
        }
    }
}

/// Creates a chrono `DateTime` from a Hacker News Unix timestamp.
pub fn datetime_from_hn_time(time: HnItemDateScalar) -> DateTime<Utc> {
    let timestamp = time as i64;
    let naive = NaiveDateTime::from_timestamp(timestamp, 0);
    DateTime::from_utc(naive, Utc)
}

/// Convert HTML to plain text, to be displayed in the terminal UI.
pub fn html_to_plain_text(html: &str, width: usize) -> String {
    html2text::from_read(html.as_bytes(), width)
}

/// Open a link in a new browser tab.
pub fn open_browser_tab(url: &str) {
    let _ = webbrowser::open(url);
}

#[cfg(test)]
mod tests {
    use super::{datetime_from_hn_time, ItemWithId, StatefulList};

    #[derive(Clone)]
    struct StatefulListTestScalar {
        index: u32,
    }

    impl StatefulListTestScalar {
        pub fn new(index: u32) -> Self {
            Self { index }
        }
    }

    impl ItemWithId<u32> for StatefulListTestScalar {
        fn get_id(&self) -> u32 {
            self.index
        }
    }

    #[test]
    pub fn test_stateful_list_wrapper() {
        let mut stateful_list = StatefulList::with_items(vec![
            StatefulListTestScalar::new(0),
            StatefulListTestScalar::new(1),
        ]);
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

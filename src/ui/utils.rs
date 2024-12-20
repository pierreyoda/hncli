use chrono::{DateTime, Utc};
use num_traits::Num;

use crate::{
    api::types::HnItemDateScalar,
    errors::{HnCliError, Result},
};

pub mod breakpoints;
pub mod debouncer;
pub mod loader;

pub trait ItemWithId<N: Copy + Num + Ord> {
    fn get_id(&self) -> N;
}

/// Creates a chrono `DateTime` from a Hacker News Unix timestamp.
pub fn datetime_from_hn_time(time: HnItemDateScalar) -> Result<DateTime<Utc>> {
    let timestamp = time as i64;
    DateTime::from_timestamp(timestamp, 0).ok_or(HnCliError::ChronoError(time))
}

/// Convert HTML to plain text, to be displayed in the terminal UI.
///
/// NB: we need the width here, so components cannot really cache this operation.
pub fn html_to_plain_text(html: &str, width: usize) -> Result<String> {
    html2text::from_read(html.as_bytes(), width).map_err(HnCliError::Html2TextError)
}

/// Open a link in a new browser tab.
pub fn open_browser_tab(url: &str) {
    let _ = webbrowser::open(url);
}

#[cfg(test)]
mod tests {
    use super::datetime_from_hn_time;

    #[test]
    pub fn test_datetime_from_hn_time() {
        let date = datetime_from_hn_time(1203647620).unwrap();
        let formatted_date = date.format("%Y-%m-%d %H:%M:%S").to_string();

        assert_eq!(formatted_date, "2008-02-22 02:33:40".to_string());
    }
}

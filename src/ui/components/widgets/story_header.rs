use tui::{buffer::Buffer, layout::Rect, style::Style, widgets::Widget};

use crate::ui::components::stories::DisplayableHackerNewsItem;

/// `StoryDetails` widget.
///
/// ```md
/// ___________________________________________
/// |                                         |
/// |                <TITLE>                  |
/// |            <URL HOSTNAME?>              |
/// |      <SCORE> POINTS / BY <USERNAME>     |
/// |   <#COMMENTS COUNT>  / POSTED <X> AGO   |
/// |_________________________________________|
/// ```
pub struct StoryHeader {
    /// Base `Style` for the widget.
    style: Style,
    /// The item to display.
    item: DisplayableHackerNewsItem,
}

impl StoryHeader {
    pub fn new(item: DisplayableHackerNewsItem) -> Self {
        Self {
            style: Style::default(),
            item,
        }
    }
}

impl Widget for StoryHeader {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);

        let start = area.left() + 2;
        for (i, title_letter) in self.item.title.chars().enumerate() {
            buf.get_mut(start + i as u16, 1).set_symbol(&"t");
        }
    }
}

use tui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    widgets::{Block, Paragraph, Widget},
};

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
pub struct StoryHeader<'a> {
    /// Base `Style` for the widget.
    style: Style,
    /// The item to display.
    item: DisplayableHackerNewsItem,
    /// (Optional) Block to render the widget inside.
    block: Option<Block<'a>>,
}

impl<'a> StoryHeader<'a> {
    pub fn new(item: DisplayableHackerNewsItem) -> Self {
        Self {
            style: Style::default(),
            item,
            block: None,
        }
    }
}

impl<'a> Widget for StoryHeader<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);

        let widget_area = match self.block.take() {
            Some(block) => {
                let inner_area = block.inner(area);
                block.render(area, buf);
                inner_area
            }
            None => area,
        };

        for x in area.left()..area.right() {
            buf.get_mut(x, area.top()).set_symbol(&"‾");
            buf.get_mut(x, area.bottom()).set_symbol(&"_");
        }

        let start = area.left() + 2;
        // for (x, title_letter) in self.item.title.chars().enumerate() {
        //     buf.get_mut(start + x as u16, area.top() + 2)
        //         .set_symbol(title_letter.to_string().as_str());
        // }
        let title_paragraph = Paragraph::new(self.item.title).alignment(Alignment::Center);
    }
}

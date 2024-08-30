//! Custom list `tui-rs` widget implementation, allowing custom list item rendering.
//!
//! tui-rs license:
//!
//! The MIT License (MIT)
//!
//! Copyright (c) 2016 Florian Dehau
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.

use std::{collections::HashMap, hash::Hash};
use unicode_width::UnicodeWidthStr;

use num_traits::Num;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{Block, ListDirection, Widget},
};

use crate::ui::utils::ItemWithId;

/// Custom `CustomList` state analogous to `ListState` but providing wrap-around navigation,
/// and storing the "raw" list items (from which each line is rendered).
#[derive(Debug)]
pub struct CustomListState<N, T>
where
    N: Copy + Num + Ord + Default,
    T: Clone + ItemWithId<N>,
{
    offset: usize,
    selected: Option<usize>,
    items: Vec<T>,
    // NB: this field is only there to prevent the "N parameter not used" compilation error
    _n: N,
}

impl<N, T> CustomListState<N, T>
where
    N: Copy + Num + Ord + Default,
    T: Clone + ItemWithId<N>,
{
    /// Create a new `StatefulList<T>` with the given items.
    pub fn with_items(items: Vec<T>) -> CustomListState<N, T> {
        Self {
            offset: 0,
            selected: None,
            items,
            _n: Default::default(),
        }
    }

    /// Is the list empty?
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get the stored items.
    pub fn get_items(&self) -> &Vec<T> {
        &self.items
    }

    /// CLear the stored items.
    pub fn clear(&mut self) {
        self.items.clear();
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
        self.select(Some(match self.selected {
            None => 0,
            Some(i) => (i + 1) % self.items.len(),
        }));
    }

    /// Select the previous item, starting at 0 if none is selected or
    /// wrapping around to `items.len() - 1` if at the start of the list.
    pub fn previous(&mut self) {
        self.select(Some(match self.selected {
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

    pub fn selected(&self) -> &Option<usize> {
        &self.selected
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.selected = index;
        if index.is_none() {
            self.offset = 0;
        }
    }

    fn reconciliate_current_selection(&mut self, old_items: Vec<T>) {
        let mut found = false;
        if let Some(selected_index) = self.selected {
            if let Some(old_selected_item) = old_items.get(selected_index) {
                let old_selected_item_id = old_selected_item.get_id();
                if let Some(new_selected_index) = self
                    .items
                    .iter()
                    .position(|i| i.get_id() == old_selected_item_id)
                {
                    found = true;
                    self.select(Some(new_selected_index));
                }
            }
        }
        if !found {
            self.select(None);
        }
    }
}

/// Custom widget working like tui-rs' `List` but with custom lines rendering.
#[derive(Debug)]
pub struct CustomList<'a, F, H, N, T>
where
    F: Fn(Rect, &mut Buffer, &T, bool),
    H: Fn(&T) -> usize,
    N: Copy + Num + Ord + Hash + Default,
    T: Clone + ItemWithId<N>,
{
    block: Option<Block<'a>>,
    /// Style used as a base style for the widget.
    style: Style,
    /// List display direction.
    direction: ListDirection,
    /// Style used to render selected item.
    highlight_style: Style,
    /// Symbol in front of the selected item (shift all items to the right)?
    highlight_symbol: Option<&'a str>,
    /// Custom state.
    state: &'a mut CustomListState<N, T>,
    /// Custom item-rendering function.
    ///
    /// Signature: (rect, buffer, line_item, is_selected).
    render: F,
    /// Get the height of a given item.
    get_item_height: H,
}

impl<'a, F, H, N, T> CustomList<'a, F, H, N, T>
where
    F: Fn(Rect, &mut Buffer, &T, bool),
    H: Fn(&T) -> usize,
    N: Copy + Num + Ord + Hash + Default,
    T: Clone + ItemWithId<N>,
{
    pub fn new(state: &'a mut CustomListState<N, T>, render: F, get_item_height: H) -> Self {
        Self {
            block: None,
            style: Style::default(),
            direction: ListDirection::default(),
            highlight_style: Style::default(),
            highlight_symbol: None,
            state,
            render,
            get_item_height,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn highlight_symbol(mut self, highlight_symbol: &'a str) -> Self {
        self.highlight_symbol = Some(highlight_symbol);
        self
    }

    pub fn highlight_style(mut self, style: Style) -> Self {
        self.highlight_style = style;
        self
    }

    fn get_items_bounds(&self, max_height: usize) -> (usize, usize, HashMap<N, usize>) {
        let offset = self
            .state
            .offset
            .min(self.state.items.len().saturating_sub(1));
        let mut start = offset;
        let mut end = offset;
        let mut height = 0;
        let mut item_heights = HashMap::new();
        for item in self.state.items.iter().skip(offset) {
            let item_height = (self.get_item_height)(item);
            if height + item_height > max_height {
                break;
            }
            height += item_height;
            item_heights.insert(item.get_id(), item_height); // caching
            end += 1;
        }

        let mut access_item_height = |item: &T| -> usize {
            if let Some(h) = item_heights.get(&item.get_id()) {
                *h
            } else {
                let h = (self.get_item_height)(item);
                item_heights.insert(item.get_id(), h);
                h
            }
        };

        let selected = self
            .state
            .selected
            .unwrap_or(0)
            .min(self.state.items.len() - 1);
        while selected >= end {
            height = height.saturating_add(access_item_height(&self.state.items[end]));
            end += 1;
            while height > max_height {
                height = height.saturating_sub(access_item_height(&self.state.items[start]));
                start += 1;
            }
        }
        while selected < start {
            start -= 1;
            height = height.saturating_add(access_item_height(&self.state.items[start]));
            while height > max_height {
                end -= 1;
                height = height.saturating_sub(access_item_height(&self.state.items[end]));
            }
        }

        (start, end, item_heights)
    }
}

impl<'a, F, H, N, T> Widget for CustomList<'a, F, H, N, T>
where
    F: Fn(Rect, &mut Buffer, &T, bool),
    H: Fn(&T) -> usize,
    N: Copy + Num + Ord + Hash + Default,
    T: Clone + ItemWithId<N>,
{
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let list_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        if list_area.width < 1 || list_area.height < 1 {
            return;
        }

        if self.state.items.is_empty() {
            return;
        }
        let list_height = list_area.height as usize;

        let (start, end, mut item_heights) = self.get_items_bounds(list_height);
        self.state.offset = start;

        let mut access_item_height = |item: &T| -> usize {
            if let Some(h) = item_heights.get(&item.get_id()) {
                *h
            } else {
                let h = (self.get_item_height)(item);
                item_heights.insert(item.get_id(), h);
                h
            }
        };

        let highlight_symbol = self.highlight_symbol.unwrap_or("");
        let blank_symbol = " ".repeat(highlight_symbol.width());

        let mut current_height = 0;
        let has_selection = self.state.selected.is_some();
        for (i, item) in self
            .state
            .items
            .iter_mut()
            .enumerate()
            .skip(self.state.offset)
            .take(end - start)
        {
            let item_height = access_item_height(item);
            let (x, y) = if self.direction == ListDirection::BottomToTop {
                current_height += item_height as u16;
                (list_area.left(), list_area.bottom() - current_height)
            } else {
                let pos = (list_area.left(), list_area.top() + current_height);
                current_height += item_height as u16;
                pos
            };

            let area = Rect {
                x,
                y,
                width: list_area.width,
                height: item_height as u16,
            };
            buf.set_style(area, self.style);

            let is_selected = self.state.selected.map(|s| s == i).unwrap_or(false);
            // if the item is selected, we need to display the hightlight symbol:
            // - either for the first line of the item only,
            // - or for each line of the item if the appropriate option is set
            let symbol = if is_selected {
                highlight_symbol
            } else {
                &blank_symbol
            };
            let (elem_x, _) = buf.set_stringn(x, y, symbol, list_area.width as usize, self.style);
            let (item_x, max_item_width) = if has_selection {
                (elem_x, (list_area.width - (elem_x - x)))
            } else {
                (x, list_area.width)
            };
            if is_selected {
                buf.set_style(area, self.highlight_style);
            }
            (self.render)(
                Rect::new(item_x, y, max_item_width, item_height as u16),
                buf,
                item,
                is_selected,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ui::utils::ItemWithId;

    use super::CustomListState;

    #[derive(Clone)]
    struct CustomListStateTestScalar {
        index: u32,
    }

    impl CustomListStateTestScalar {
        pub fn new(index: u32) -> Self {
            Self { index }
        }
    }

    impl ItemWithId<u32> for CustomListStateTestScalar {
        fn get_id(&self) -> u32 {
            self.index
        }
    }

    #[test]
    pub fn test_stateful_custom_list_wrapper() {
        let mut stateful_list = CustomListState::with_items(vec![
            CustomListStateTestScalar::new(0),
            CustomListStateTestScalar::new(1),
        ]);
        assert_eq!(*stateful_list.selected(), None);

        stateful_list.next();
        assert_eq!(*stateful_list.selected(), Some(0));
        stateful_list.next();
        assert_eq!(*stateful_list.selected(), Some(1));
        stateful_list.next();
        assert_eq!(*stateful_list.selected(), Some(0));

        stateful_list.previous();
        assert_eq!(*stateful_list.selected(), Some(1));
        stateful_list.previous();
        assert_eq!(*stateful_list.selected(), Some(0));

        stateful_list.select(None);
        assert_eq!(*stateful_list.selected(), None);
        stateful_list.previous();
        assert_eq!(*stateful_list.selected(), Some(0));
    }
}

#[cfg(test)]
mod tests {
  use crate::models::stateful_list::StatefulList;
  use crate::ui::styles::ManagarrStyle;
  use crate::ui::utils::{layout_block, title_block};
  use crate::ui::widgets::selectable_list::SelectableList;
  use pretty_assertions::assert_eq;
  use ratatui::style::{Style, Stylize};
  use ratatui::widgets::ListItem;

  #[test]
  fn test_selectable_list_new() {
    let items = vec!["test"];
    let mut stateful_list = StatefulList::default();
    stateful_list.set_items(items.clone());

    let selectable_list =
      SelectableList::new(&mut stateful_list, |item| ListItem::new(item.to_string()));

    let row_mapper = selectable_list.row_mapper;
    assert_eq!(selectable_list.content.items, items);
    assert_eq!(row_mapper(&"test"), ListItem::new("test"));
    assert_eq!(selectable_list.highlight_style, Style::new().highlight());
    assert_eq!(selectable_list.block, layout_block());
  }

  #[test]
  fn test_selectable_list_highlight_style() {
    let items = vec!["test"];
    let mut stateful_list = StatefulList::default();
    stateful_list.set_items(items.clone());

    let selectable_list =
      SelectableList::new(&mut stateful_list, |item| ListItem::new(item.to_string()))
        .highlight_style(Style::new().bold());

    let row_mapper = selectable_list.row_mapper;
    assert_eq!(selectable_list.highlight_style, Style::new().bold());
    assert_eq!(selectable_list.content.items, items);
    assert_eq!(row_mapper(&"test"), ListItem::new("test"));
    assert_eq!(selectable_list.block, layout_block());
  }

  #[test]
  fn test_selectable_list_block() {
    let items = vec!["test"];
    let mut stateful_list = StatefulList::default();
    stateful_list.set_items(items.clone());

    let selectable_list =
      SelectableList::new(&mut stateful_list, |item| ListItem::new(item.to_string()))
        .block(title_block("test"));

    let row_mapper = selectable_list.row_mapper;
    assert_eq!(selectable_list.block, title_block("test"));
    assert_eq!(selectable_list.content.items, items);
    assert_eq!(row_mapper(&"test"), ListItem::new("test"));
    assert_eq!(selectable_list.highlight_style, Style::new().highlight());
  }
}

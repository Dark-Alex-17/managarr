#[cfg(test)]
mod tests {
  use crate::models::stateful_list::StatefulList;
  use crate::ui::widgets::selectable_list::SelectableList;
  use pretty_assertions::assert_eq;
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
  }
}

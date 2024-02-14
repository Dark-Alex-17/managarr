#[cfg(test)]
mod tests {
  use crate::models::stateful_list::StatefulList;
  use crate::models::stateful_table::{SortOption, StatefulTable};
  use crate::models::Scrollable;
  use crate::ui::utils::layout_block;
  use crate::ui::widgets::managarr_table::ManagarrTable;
  use pretty_assertions::assert_eq;
  use ratatui::layout::{Alignment, Constraint};
  use ratatui::text::Text;
  use ratatui::widgets::{Block, Cell, Row};

  #[test]
  fn test_managarr_table_new() {
    let items = vec!["item1", "item2", "item3"];
    let mut stateful_table = StatefulTable::default();
    stateful_table.set_items(items.clone());

    let managarr_table =
      ManagarrTable::new(Some(&mut stateful_table), |&s| Row::new(vec![Cell::new(s)]));

    let row_mapper = managarr_table.row_mapper;
    assert_eq!(managarr_table.content.unwrap().items, items);
    assert_eq!(row_mapper(&"item1"), Row::new(vec![Cell::new("item1")]));
    assert_eq!(managarr_table.table_headers, Vec::<String>::new());
    assert_eq!(managarr_table.constraints, Vec::new());
    assert_eq!(managarr_table.footer, None);
    assert_eq!(managarr_table.footer_alignment, Alignment::Left);
    assert_eq!(managarr_table.block, Block::new());
    assert_eq!(managarr_table.margin, 0);
    assert!(!managarr_table.is_loading);
    assert!(managarr_table.highlight_rows);
    assert!(!managarr_table.is_sorting);
  }

  #[test]
  fn test_managarr_table_headers() {
    let items = vec!["item1", "item2", "item3"];
    let mut stateful_table = StatefulTable::default();
    stateful_table.set_items(items.clone());
    let headers = ["column 1", "column 2"];

    let managarr_table =
      ManagarrTable::new(Some(&mut stateful_table), |&s| Row::new(vec![Cell::new(s)]))
        .headers(headers);

    let row_mapper = managarr_table.row_mapper;
    assert_eq!(managarr_table.table_headers, headers);
    assert_eq!(managarr_table.content.unwrap().items, items);
    assert_eq!(row_mapper(&"item1"), Row::new(vec![Cell::new("item1")]));
    assert_eq!(managarr_table.constraints, Vec::new());
    assert_eq!(managarr_table.footer, None);
    assert_eq!(managarr_table.footer_alignment, Alignment::Left);
    assert_eq!(managarr_table.block, Block::new());
    assert_eq!(managarr_table.margin, 0);
    assert!(!managarr_table.is_loading);
    assert!(managarr_table.highlight_rows);
    assert!(!managarr_table.is_sorting);
  }

  #[test]
  fn test_managarr_table_constraints() {
    let items = vec!["item1", "item2", "item3"];
    let mut stateful_table = StatefulTable::default();
    stateful_table.set_items(items.clone());
    let constraints = [Constraint::Length(1), Constraint::Fill(1)];

    let managarr_table =
      ManagarrTable::new(Some(&mut stateful_table), |&s| Row::new(vec![Cell::new(s)]))
        .constraints(constraints);

    let row_mapper = managarr_table.row_mapper;
    assert_eq!(managarr_table.constraints, constraints);
    assert_eq!(managarr_table.content.unwrap().items, items);
    assert_eq!(row_mapper(&"item1"), Row::new(vec![Cell::new("item1")]));
    assert_eq!(managarr_table.table_headers, Vec::<String>::new());
    assert_eq!(managarr_table.footer, None);
    assert_eq!(managarr_table.footer_alignment, Alignment::Left);
    assert_eq!(managarr_table.block, Block::new());
    assert_eq!(managarr_table.margin, 0);
    assert!(!managarr_table.is_loading);
    assert!(managarr_table.highlight_rows);
    assert!(!managarr_table.is_sorting);
  }

  #[test]
  fn test_managarr_table_footer() {
    let items = vec!["item1", "item2", "item3"];
    let mut stateful_table = StatefulTable::default();
    stateful_table.set_items(items.clone());
    let footer = "footer".to_owned();

    let managarr_table =
      ManagarrTable::new(Some(&mut stateful_table), |&s| Row::new(vec![Cell::new(s)]))
        .footer(Some(footer.clone()));

    let row_mapper = managarr_table.row_mapper;
    assert_eq!(managarr_table.footer, Some(footer));
    assert_eq!(managarr_table.content.unwrap().items, items);
    assert_eq!(row_mapper(&"item1"), Row::new(vec![Cell::new("item1")]));
    assert_eq!(managarr_table.table_headers, Vec::<String>::new());
    assert_eq!(managarr_table.constraints, Vec::new());
    assert_eq!(managarr_table.footer_alignment, Alignment::Left);
    assert_eq!(managarr_table.block, Block::new());
    assert_eq!(managarr_table.margin, 0);
    assert!(!managarr_table.is_loading);
    assert!(managarr_table.highlight_rows);
    assert!(!managarr_table.is_sorting);
  }

  #[test]
  fn test_managarr_table_footer_alignment() {
    let items = vec!["item1", "item2", "item3"];
    let mut stateful_table = StatefulTable::default();
    stateful_table.set_items(items.clone());

    let managarr_table =
      ManagarrTable::new(Some(&mut stateful_table), |&s| Row::new(vec![Cell::new(s)]))
        .footer_alignment(Alignment::Center);

    let row_mapper = managarr_table.row_mapper;
    assert_eq!(managarr_table.footer_alignment, Alignment::Center);
    assert_eq!(managarr_table.content.unwrap().items, items);
    assert_eq!(row_mapper(&"item1"), Row::new(vec![Cell::new("item1")]));
    assert_eq!(managarr_table.table_headers, Vec::<String>::new());
    assert_eq!(managarr_table.constraints, Vec::new());
    assert_eq!(managarr_table.footer, None);
    assert_eq!(managarr_table.block, Block::new());
    assert_eq!(managarr_table.margin, 0);
    assert!(!managarr_table.is_loading);
    assert!(managarr_table.highlight_rows);
    assert!(!managarr_table.is_sorting);
  }

  #[test]
  fn test_managarr_table_block() {
    let items = vec!["item1", "item2", "item3"];
    let mut stateful_table = StatefulTable::default();
    stateful_table.set_items(items.clone());

    let managarr_table =
      ManagarrTable::new(Some(&mut stateful_table), |&s| Row::new(vec![Cell::new(s)]))
        .block(layout_block());

    let row_mapper = managarr_table.row_mapper;
    assert_eq!(managarr_table.block, layout_block());
    assert_eq!(managarr_table.content.unwrap().items, items);
    assert_eq!(row_mapper(&"item1"), Row::new(vec![Cell::new("item1")]));
    assert_eq!(managarr_table.table_headers, Vec::<String>::new());
    assert_eq!(managarr_table.constraints, Vec::new());
    assert_eq!(managarr_table.footer, None);
    assert_eq!(managarr_table.footer_alignment, Alignment::Left);
    assert_eq!(managarr_table.margin, 0);
    assert!(!managarr_table.is_loading);
    assert!(managarr_table.highlight_rows);
    assert!(!managarr_table.is_sorting);
  }

  #[test]
  fn test_managarr_table_margin() {
    let items = vec!["item1", "item2", "item3"];
    let mut stateful_table = StatefulTable::default();
    stateful_table.set_items(items.clone());

    let managarr_table =
      ManagarrTable::new(Some(&mut stateful_table), |&s| Row::new(vec![Cell::new(s)])).margin(1);

    let row_mapper = managarr_table.row_mapper;
    assert_eq!(managarr_table.margin, 1);
    assert_eq!(managarr_table.content.unwrap().items, items);
    assert_eq!(row_mapper(&"item1"), Row::new(vec![Cell::new("item1")]));
    assert_eq!(managarr_table.table_headers, Vec::<String>::new());
    assert_eq!(managarr_table.constraints, Vec::new());
    assert_eq!(managarr_table.footer, None);
    assert_eq!(managarr_table.footer_alignment, Alignment::Left);
    assert_eq!(managarr_table.block, Block::new());
    assert!(!managarr_table.is_loading);
    assert!(managarr_table.highlight_rows);
    assert!(!managarr_table.is_sorting);
  }

  #[test]
  fn test_managarr_table_loading() {
    let items = vec!["item1", "item2", "item3"];
    let mut stateful_table = StatefulTable::default();
    stateful_table.set_items(items.clone());

    let managarr_table =
      ManagarrTable::new(Some(&mut stateful_table), |&s| Row::new(vec![Cell::new(s)]))
        .loading(true);

    let row_mapper = managarr_table.row_mapper;
    assert!(managarr_table.is_loading);
    assert_eq!(managarr_table.content.unwrap().items, items);
    assert_eq!(row_mapper(&"item1"), Row::new(vec![Cell::new("item1")]));
    assert_eq!(managarr_table.table_headers, Vec::<String>::new());
    assert_eq!(managarr_table.constraints, Vec::new());
    assert_eq!(managarr_table.footer, None);
    assert_eq!(managarr_table.footer_alignment, Alignment::Left);
    assert_eq!(managarr_table.block, Block::new());
    assert_eq!(managarr_table.margin, 0);
    assert!(managarr_table.highlight_rows);
    assert!(!managarr_table.is_sorting);
  }

  #[test]
  fn test_managarr_table_highlight_rows() {
    let items = vec!["item1", "item2", "item3"];
    let mut stateful_table = StatefulTable::default();
    stateful_table.set_items(items.clone());

    let managarr_table =
      ManagarrTable::new(Some(&mut stateful_table), |&s| Row::new(vec![Cell::new(s)]))
        .highlight_rows(false);

    let row_mapper = managarr_table.row_mapper;
    assert!(!managarr_table.highlight_rows);
    assert_eq!(managarr_table.content.unwrap().items, items);
    assert_eq!(row_mapper(&"item1"), Row::new(vec![Cell::new("item1")]));
    assert_eq!(managarr_table.table_headers, Vec::<String>::new());
    assert_eq!(managarr_table.constraints, Vec::new());
    assert_eq!(managarr_table.footer, None);
    assert_eq!(managarr_table.footer_alignment, Alignment::Left);
    assert_eq!(managarr_table.block, Block::new());
    assert_eq!(managarr_table.margin, 0);
    assert!(!managarr_table.is_loading);
    assert!(!managarr_table.is_sorting);
  }

  #[test]
  fn test_managarr_table_sorting() {
    let items = vec!["item1", "item2", "item3"];
    let mut stateful_table = StatefulTable::default();
    stateful_table.set_items(items.clone());

    let managarr_table =
      ManagarrTable::new(Some(&mut stateful_table), |&s| Row::new(vec![Cell::new(s)]))
        .sorting(true);

    let row_mapper = managarr_table.row_mapper;
    assert!(managarr_table.is_sorting);
    assert_eq!(managarr_table.content.unwrap().items, items);
    assert_eq!(row_mapper(&"item1"), Row::new(vec![Cell::new("item1")]));
    assert_eq!(managarr_table.table_headers, Vec::<String>::new());
    assert_eq!(managarr_table.constraints, Vec::new());
    assert_eq!(managarr_table.footer, None);
    assert_eq!(managarr_table.footer_alignment, Alignment::Left);
    assert_eq!(managarr_table.block, Block::new());
    assert_eq!(managarr_table.margin, 0);
    assert!(!managarr_table.is_loading);
    assert!(managarr_table.highlight_rows);
  }

  #[test]
  fn test_managarr_table_parse_headers() {
    let items = vec!["item1", "item2", "item3"];
    let mut sort_list = StatefulList::default();
    sort_list.set_items(vec![
      SortOption {
        name: "column 1",
        cmp_fn: None,
      },
      SortOption {
        name: "column 2",
        cmp_fn: None,
      },
    ]);
    sort_list.scroll_down();
    let mut stateful_table = StatefulTable::default();
    stateful_table.set_items(items.clone());
    stateful_table.sort = Some(sort_list);
    let headers = ["column 1", "column 2"];

    let managarr_table =
      ManagarrTable::new(Some(&mut stateful_table), |&s| Row::new(vec![Cell::new(s)]))
        .headers(headers);

    assert_eq!(
      managarr_table.parse_headers(),
      vec![Text::from("column 1"), Text::from("column 2 ▼")]
    );
  }

  #[test]
  fn test_managarr_table_parse_headers_ascending() {
    let items = vec!["item1", "item2", "item3"];
    let mut sort_list = StatefulList::default();
    sort_list.set_items(vec![
      SortOption {
        name: "column 1",
        cmp_fn: None,
      },
      SortOption {
        name: "column 2",
        cmp_fn: None,
      },
    ]);
    sort_list.scroll_down();
    let mut stateful_table = StatefulTable::default();
    stateful_table.set_items(items.clone());
    stateful_table.sort = Some(sort_list);
    stateful_table.sort_asc = true;
    let headers = ["column 1", "column 2"];

    let managarr_table =
      ManagarrTable::new(Some(&mut stateful_table), |&s| Row::new(vec![Cell::new(s)]))
        .headers(headers);

    assert_eq!(
      managarr_table.parse_headers(),
      vec![Text::from("column 1"), Text::from("column 2 ▲")]
    );
  }
}

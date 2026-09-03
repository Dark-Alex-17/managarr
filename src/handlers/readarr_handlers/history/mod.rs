use crate::models::readarr_models::ReadarrHistoryItem;
use crate::models::stateful_table::SortOption;

#[cfg(test)]
#[path = "history_handler_tests.rs"]
mod history_handler_tests;

pub(in crate::handlers::readarr_handlers) fn history_sorting_options()
-> Vec<SortOption<ReadarrHistoryItem>> {
  vec![
    SortOption {
      name: "Source Title",
      cmp_fn: Some(|a, b| {
        a.source_title
          .text
          .to_lowercase()
          .cmp(&b.source_title.text.to_lowercase())
      }),
    },
    SortOption {
      name: "Event Type",
      cmp_fn: Some(|a, b| {
        a.event_type
          .to_string()
          .to_lowercase()
          .cmp(&b.event_type.to_string().to_lowercase())
      }),
    },
    SortOption {
      name: "Quality",
      cmp_fn: Some(|a, b| {
        a.quality
          .quality
          .name
          .to_lowercase()
          .cmp(&b.quality.quality.name.to_lowercase())
      }),
    },
    SortOption {
      name: "Date",
      cmp_fn: Some(|a, b| a.date.cmp(&b.date)),
    },
  ]
}

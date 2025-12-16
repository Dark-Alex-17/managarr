#[macro_export]
macro_rules! sort_option {
  ($field:ident) => {
    SortOption {
      name: "Something",
      cmp_fn: Some(|a, b| a.$field.cmp(&b.$field)),
    }
  };
}

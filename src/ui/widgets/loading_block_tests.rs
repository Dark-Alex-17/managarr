#[cfg(test)]
mod tests {
  use crate::ui::utils::layout_block;
  use crate::ui::widgets::loading_block::LoadingBlock;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_loading_block_new() {
    let loading_block = LoadingBlock::new(true, layout_block());

    assert_eq!(loading_block.is_loading, true);
    assert_eq!(loading_block.block, layout_block());
  }
}

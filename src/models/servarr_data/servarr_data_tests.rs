#[cfg(test)]
mod tests {
  use crate::models::Route;
  use crate::models::servarr_data::ActiveKeybindingBlock;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_from_active_keybinding_block_to_route() {
    assert_eq!(Route::from(ActiveKeybindingBlock::Help), Route::Keybindings);
  }
}

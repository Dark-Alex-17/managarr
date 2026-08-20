#[cfg(test)]
mod tests {
  use crate::app::App;
  use crate::models::servarr_data::readarr::readarr_data::ActiveReadarrBlock;
  use pretty_assertions::assert_eq;

  #[tokio::test]
  async fn test_dispatch_by_readarr_block_resets_the_tick_count() {
    let mut app = App {
      tick_count: 2,
      ..App::test_default()
    };

    app
      .dispatch_by_readarr_block(&ActiveReadarrBlock::Authors)
      .await;

    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_readarr_block_clears_the_pending_prompt_confirmation() {
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;

    app
      .dispatch_by_readarr_block(&ActiveReadarrBlock::Authors)
      .await;

    assert!(!app.data.readarr_data.prompt_confirm);
    assert!(!app.should_refresh);
  }

  #[tokio::test]
  async fn test_check_for_readarr_prompt_action_no_prompt_confirm() {
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = false;

    app.check_for_readarr_prompt_action().await;

    assert!(!app.data.readarr_data.prompt_confirm);
    assert!(!app.should_refresh);
  }

  #[tokio::test]
  async fn test_check_for_readarr_prompt_action_with_no_prompt_confirm_action() {
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;

    app.check_for_readarr_prompt_action().await;

    assert!(!app.data.readarr_data.prompt_confirm);
    assert_none!(app.data.readarr_data.prompt_confirm_action);
    assert!(!app.should_refresh);
  }

  #[tokio::test]
  async fn test_readarr_on_tick_first_render() {
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;
    app.is_first_render = true;

    app.readarr_on_tick(ActiveReadarrBlock::Downloads).await;

    assert!(!app.data.readarr_data.prompt_confirm);
    assert!(!app.is_first_render);
  }

  #[tokio::test]
  async fn test_readarr_on_tick_routing() {
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;
    app.is_routing = true;
    app.should_refresh = true;
    app.is_first_render = false;
    app.tick_count = 1;

    app.readarr_on_tick(ActiveReadarrBlock::Downloads).await;

    assert!(!app.data.readarr_data.prompt_confirm);
    assert!(!app.cancellation_token.is_cancelled());
  }

  #[tokio::test]
  async fn test_readarr_on_tick_routing_while_long_request_is_running_should_cancel_request() {
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;
    app.is_routing = true;
    app.should_refresh = false;
    app.is_first_render = false;
    app.tick_count = 1;

    app.readarr_on_tick(ActiveReadarrBlock::Downloads).await;

    assert!(app.cancellation_token.is_cancelled());
  }

  #[tokio::test]
  async fn test_readarr_on_tick_should_refresh() {
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;
    app.should_refresh = true;
    app.is_first_render = false;
    app.tick_count = 1;

    app.readarr_on_tick(ActiveReadarrBlock::Downloads).await;

    assert!(app.should_refresh);
    assert!(!app.data.readarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }
}

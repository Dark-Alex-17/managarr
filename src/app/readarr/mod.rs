use super::App;
use crate::models::servarr_data::readarr::readarr_data::ActiveReadarrBlock;

pub mod readarr_context_clues;

#[cfg(test)]
#[path = "readarr_tests.rs"]
mod readarr_tests;

impl App<'_> {
  pub(super) async fn dispatch_by_readarr_block(
    &mut self,
    _active_readarr_block: &ActiveReadarrBlock,
  ) {
    self.check_for_readarr_prompt_action().await;
    self.reset_tick_count();
  }

  async fn check_for_readarr_prompt_action(&mut self) {
    if self.data.readarr_data.prompt_confirm {
      self.data.readarr_data.prompt_confirm = false;
      if let Some(readarr_event) = self.data.readarr_data.prompt_confirm_action.take() {
        self.dispatch_network_event(readarr_event.into()).await;
        self.should_refresh = true;
      }
    }
  }

  pub(super) async fn readarr_on_tick(&mut self, active_readarr_block: ActiveReadarrBlock) {
    if self.is_first_render {
      self.dispatch_by_readarr_block(&active_readarr_block).await;
      self.is_first_render = false;
      return;
    }

    if self.should_refresh {
      self.dispatch_by_readarr_block(&active_readarr_block).await;
    }

    if self.is_routing {
      if !self.should_refresh {
        self.cancellation_token.cancel();
      } else {
        self.dispatch_by_readarr_block(&active_readarr_block).await;
      }
    }
  }
}

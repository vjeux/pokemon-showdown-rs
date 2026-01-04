use crate::*;
use crate::event::EventResult;

impl Battle {

    /// Clear the currently active move
    /// Equivalent to battle.ts clearActiveMove()
    //
    // 	clearActiveMove(failed?: boolean) {
    // 		if (this.activeMove) {
    // 			if (!failed) {
    // 				this.lastMove = this.activeMove;
    // 			}
    // 			this.activeMove = null;
    // 			this.activePokemon = null;
    // 			this.activeTarget = null;
    // 		}
    // 	}
    //
    pub fn clear_active_move(&mut self, failed: bool) {
        if self.active_move.is_some() {
            if !failed {
                self.last_move = self.active_move.clone();
            }
            self.active_move = None;
            self.active_pokemon = None;
            self.active_target = None;
        }
    }
}

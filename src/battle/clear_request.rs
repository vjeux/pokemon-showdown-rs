use crate::*;
use crate::battle::BattleRequestState;

impl Battle {

    /// Clear request state
    /// Equivalent to battle.ts clearRequest() (battle.ts:1364-1370)
    //
    // 	clearRequest() {
    // 		this.requestState = '';
    // 		for (const side of this.sides) {
    // 			side.activeRequest = null;
    // 			side.clearChoice();
    // 		}
    // 	}
    //
    pub fn clear_request(&mut self) {
        // JavaScript: this.requestState = '';
        self.request_state = BattleRequestState::None;

        // JavaScript: for (const side of this.sides) { side.activeRequest = null; side.clearChoice(); }
        for side in &mut self.sides {
            side.request_state = crate::side::RequestState::None;
            side.clear_choice(self.request_state);
        }
    }
}

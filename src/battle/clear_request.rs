use crate::*;
use crate::battle::BattleRequestState;
use crate::choice::Choice;
use crate::side::RequestState;

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
            side.request_state = RequestState::None;
            side.choice = crate::side::Choice::new(); // clearChoice()
        }
    }
}

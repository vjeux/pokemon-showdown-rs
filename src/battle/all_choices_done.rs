use crate::*;

impl Battle {

    /// Check if all choices are done
    /// Equivalent to battle.ts allChoicesDone()
    /// Check if all players have made their choices
    /// Equivalent to battle.ts allChoicesDone() (battle.ts:3059-3068)
    ///
    // TypeScript source:
    // /**
    // 	 * returns true if both decisions are complete
    // 	 */
    // 	allChoicesDone() {
    // 		let totalActions = 0;
    // 		for (const side of this.sides) {
    // 			if (side.isChoiceDone()) {
    // 				if (!this.supportCancel) side.choice.cantUndo = true;
    // 				totalActions++;
    // 			}
    // 		}
    // 		return totalActions >= this.sides.length;
    // 	}
    //
    pub fn all_choices_done(&mut self) -> bool {
        // JS: let totalActions = 0;
        let mut total_actions = 0;

        // Get picked_team_size from rule_table if available
        let picked_team_size = self.rule_table.as_ref().and_then(|rt| rt.picked_team_size);

        // JS: for (const side of this.sides)
        for side in &mut self.sides {
            // JS: if (side.isChoiceDone())
            if side.is_choice_done(picked_team_size) {
                // JS: if (!this.supportCancel) side.choice.cantUndo = true;
                if !self.support_cancel {
                    side.choice.cant_undo = true;
                }

                // JS: totalActions++;
                total_actions += 1;
            }
        }

        // JS: return totalActions >= this.sides.length;
        total_actions >= self.sides.len()
    }
}

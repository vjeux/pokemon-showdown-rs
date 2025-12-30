use crate::*;

impl Battle {

    /// Process a choice from a player
    /// Takes a choice string passed from the client. Starts the next
    /// turn if all required choices have been made.
    /// Equivalent to battle.ts choose()
    //
    // 	choose(sideid: SideID, input: string) {
    // 		const side = this.getSide(sideid);
    //
    // 		if (!side.choose(input)) {
    // 			if (!side.choice.error) {
    // 				side.emitChoiceError(`Unknown error for choice: ${input}. If you're not using a custom client, please report this as a bug.`);
    // 			}
    // 			return false;
    // 		}
    //
    // 		if (!side.isChoiceDone()) {
    // 			side.emitChoiceError(`Incomplete choice: ${input} - missing other pokemon`);
    // 			return false;
    // 		}
    // 		if (this.allChoicesDone()) this.commitChoices();
    // 		return true;
    // 	}
    //
    pub fn choose(&mut self, side_id: SideID, input: &str) -> bool {
        self.input_log
            .push(format!(">{} {}", side_id.to_str(), input));

        let side_idx = side_id.index();

        // JS: if (!side.choose(input))
        let choose_result = match self.sides[side_idx].choose(input) {
            Ok(success) => success,
            Err(_) => false,
        };

        if !choose_result {
            // JS: if (!side.choice.error)
            if self.sides[side_idx].choice.error.is_empty() {
                // JS: side.emitChoiceError(...)
                self.sides[side_idx].emit_choice_error(&format!(
                    "Unknown error for choice: {}. If you're not using a custom client, please report this as a bug.",
                    input
                ));
            }
            return false;
        }

        // JS: if (!side.isChoiceDone())
        if !self.sides[side_idx].is_choice_done(None) {
            // JS: side.emitChoiceError(...)
            self.sides[side_idx].emit_choice_error(&format!(
                "Incomplete choice: {} - missing other pokemon",
                input
            ));
            return false;
        }

        // JS: if (this.allChoicesDone()) this.commitChoices();
        if self.all_choices_done() {
            self.commit_choices();
        }

        true
    }
}

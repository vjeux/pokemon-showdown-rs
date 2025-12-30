use crate::*;

impl Battle {

    /// Process a choice from a player
    // TypeScript source:
    // /**
    // 	 * Takes a choice string passed from the client. Starts the next
    // 	 * turn if all required choices have been made.
    // 	 */
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
    pub fn choose(&mut self, side_id: SideID, choice: &str) -> Result<(), String> {
        let side_idx = side_id.index();
        if side_idx >= self.sides.len() {
            return Err(format!("Invalid side: {}", side_id.to_str()));
        }

        self.input_log
            .push(format!(">{} {}", side_id.to_str(), choice));

        // Check if this is a comma-separated choice (for Doubles/Triples with multiple active Pokemon)
        if choice.contains(',') {
            // Split by comma and validate each sub-choice
            let sub_choices: Vec<&str> = choice.split(',').map(|s| s.trim()).collect();

            // Validate each sub-choice individually with its pokemon_index
            for (pokemon_index, sub_choice) in sub_choices.iter().enumerate() {
                self.validate_single_choice(side_id, sub_choice, Some(pokemon_index))?;
            }
            Ok(())
        } else {
            // Single choice (Singles or single slot in Doubles/Triples)
            // In singles, pokemon_index is 0
            self.validate_single_choice(side_id, choice, Some(0))
        }
    }
}

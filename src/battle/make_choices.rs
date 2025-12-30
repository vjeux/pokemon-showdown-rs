use crate::*;

impl Battle {

    /// Make choices for both sides and run the turn
    /// This is the main entry point for progressing the battle
    // TypeScript source:
    // /**
    // 	 * Convenience method for easily making choices.
    // 	 */
    // 	makeChoices(...inputs: string[]) {
    // 		if (inputs.length) {
    // 			for (const [i, input] of inputs.entries()) {
    // 				if (input) this.sides[i].choose(input);
    // 			}
    // 		} else {
    // 			for (const side of this.sides) {
    // 				side.autoChoose();
    // 			}
    // 		}
    // 		this.commitChoices();
    // 	}
    //
    pub fn make_choices(&mut self, p1_choice: &str, p2_choice: &str) {
        // Parse and validate choices
        self.parse_choice(SideID::P1, p1_choice);
        self.parse_choice(SideID::P2, p2_choice);

        // Log choices
        if !p1_choice.is_empty() {
            self.input_log.push(format!(">p1 {}", p1_choice));
        }
        if !p2_choice.is_empty() {
            self.input_log.push(format!(">p2 {}", p2_choice));
        }

        // Run the turn
        self.commit_choices();
    }
}

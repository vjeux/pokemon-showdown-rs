use crate::*;

impl Battle {

    /// Make choices for both sides and run the turn
    /// This is the main entry point for progressing the battle
    /// Equivalent to battle.ts makeChoices()
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
    pub fn make_choices(&mut self, inputs: &[&str]) {
        // JS: if (inputs.length) {
        if !inputs.is_empty() {
            // JS: for (const [i, input] of inputs.entries()) {
            // JS:     if (input) this.sides[i].choose(input);
            // JS: }
            for (i, input) in inputs.iter().enumerate() {
                if !input.is_empty() && i < self.sides.len() {
                    let _ = self.sides[i].choose(input);
                }
            }
        } else {
            // JS: for (const side of this.sides) {
            // JS:     side.autoChoose();
            // JS: }
            for i in 0..self.sides.len() {
                self.sides[i].auto_choose();
            }
        }

        // JS: this.commitChoices();
        self.commit_choices();
    }
}


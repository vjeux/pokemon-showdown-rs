use crate::*;
use crate::side::Side;

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
                    // Need to pass battle to choose, use raw pointers to bypass borrow checker
                    unsafe {
                        let side_ptr = &mut self.sides[i] as *mut Side;
                        let battle_ptr = self as *mut Battle;
                        let _ = (*side_ptr).choose(&mut *battle_ptr, input);
                    }
                }
            }
        } else {
            // JS: for (const side of this.sides) {
            // JS:     side.autoChoose();
            // JS: }
            // Note: We need to split the borrow to pass Battle to auto_choose
            // Use raw pointers to bypass the borrow checker
            for i in 0..self.sides.len() {
                unsafe {
                    let side_ptr = &mut self.sides[i] as *mut Side;
                    let battle_ptr = self as *mut Battle;
                    (*side_ptr).auto_choose(&mut *battle_ptr);
                }
            }
        }

        // JS: this.commitChoices();
        self.commit_choices();
    }
}


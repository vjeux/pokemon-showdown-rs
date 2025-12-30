use crate::*;

impl Battle {

    /// Declare a tie
    /// Equivalent to battle.ts tie()
    //
    // 	tie() {
    // 		return this.win();
    // 	}
    //
    pub fn tie(&mut self) -> bool {
        self.win(None)
    }
}

use crate::*;

impl Battle {

    /// Check if a side can switch
    /// Equivalent to battle.ts canSwitch()
    //
    // 	canSwitch(side: Side) {
    // 		return this.possibleSwitches(side).length;
    // 	}
    //
    pub fn can_switch(&self, side_idx: usize) -> usize {
        self.possible_switches(side_idx).len()
    }
}

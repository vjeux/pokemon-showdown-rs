use crate::*;

impl Battle {

    /// Get a random switchable Pokemon
    /// Equivalent to battle.ts getRandomSwitchable()
    //
    // 	getRandomSwitchable(side: Side) {
    // 		const canSwitchIn = this.possibleSwitches(side);
    // 		return canSwitchIn.length ? this.sample(canSwitchIn) : null;
    // 	}
    //
    pub fn get_random_switchable(&mut self, side_idx: usize) -> Option<usize> {
        let switches = self.possible_switches(side_idx);
        if switches.is_empty() {
            return None;
        }
        let idx = self.random(switches.len() as i32) as usize;
        Some(switches[idx])
    }
}

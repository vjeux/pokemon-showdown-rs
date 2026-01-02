use crate::*;

impl Pokemon {

    /// Try to trap the Pokemon
    //
    // 	tryTrap(isHidden = false) {
    // 		if (!this.runStatusImmunity('trapped')) return false;
    // 		if (this.trapped && isHidden) return true;
    // 		this.trapped = isHidden ? 'hidden' : true;
    // 		return true;
    // 	}
    //
    pub fn try_trap(&mut self, is_hidden: bool) -> bool {
        // TODO: implement the same logic as JavaScript

        if self.trapped && is_hidden {
            return true;
        }
        self.trapped = true;
        true
    }
}

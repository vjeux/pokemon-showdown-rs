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
        // JS: if (!this.runStatusImmunity('trapped')) return false;
        // Note: Missing runStatusImmunity('trapped') check
        // Note: Would need Battle reference to call runStatusImmunity

        // JS: if (this.trapped && isHidden) return true;
        if self.trapped && is_hidden {
            return true;
        }

        // JS: this.trapped = isHidden ? 'hidden' : true;
        self.trapped = true;
        // Note: Rust trapped field is bool, cannot represent 'hidden' state
        // Note: JavaScript uses bool | 'hidden' to distinguish visible vs hidden trap

        // JS: return true;
        true
    }
}

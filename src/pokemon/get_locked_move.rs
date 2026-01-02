// JS Source:
// 	getLockedMove(): ID | null {
// 		const lockedMove = this.battle.runEvent('LockMove', this);
// 		return (lockedMove === true) ? null : lockedMove;
// 	}


use crate::*;

impl Pokemon {

    /// Get locked move (for multi-turn moves)
    // TypeScript source:
    // /**
    // 	 * This refers to multi-turn moves like SolarBeam and Outrage and
    // 	 * Sky Drop, which remove all choice (no dynamax, switching, etc).
    // 	 * Don't use it for "soft locks" like Choice Band.
    // 	 */
    // 	getLockedMove(): ID | null {
    // 		const lockedMove = this.battle.runEvent('LockMove', this);
    // 		return (lockedMove === true) ? null : lockedMove;
    // 	}
    //
    pub fn get_locked_move(&self) -> Option<&ID> {
        self.locked_move.as_ref()
    }
}

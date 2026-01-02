// JS Source:
//
// /**
//  * This refers to multi-turn moves like SolarBeam and Outrage and
//  * Sky Drop, which remove all choice (no dynamax, switching, etc).
//  * Don't use it for "soft locks" like Choice Band.
//  */
// getLockedMove(): ID | null {
// 	const lockedMove = this.battle.runEvent('LockMove', this);
// 	return (lockedMove === true) ? null : lockedMove;
// }
//
// Note: In Rust, this requires Battle reference to call runEvent.
// For now, returning the locked_move field directly.
// Note: Should refactor to associated function: Pokemon::get_locked_move(battle, pokemon_pos)
// Note: This pattern would match other Battle-dependent methods

use crate::*;

impl Pokemon {
    /// Get locked move (for multi-turn moves)
    /// Equivalent to pokemon.ts getLockedMove()
    ///
    /// This refers to multi-turn moves like SolarBeam and Outrage and
    /// Sky Drop, which remove all choice (no dynamax, switching, etc).
    /// Don't use it for "soft locks" like Choice Band.
    pub fn get_locked_move(&self) -> Option<&ID> {
        // JS: const lockedMove = this.battle.runEvent('LockMove', this);
        // Note: Missing battle.run_event('LockMove', pokemon_pos) call
        // Note: runEvent can modify the locked move based on abilities/items

        // JS: return (lockedMove === true) ? null : lockedMove;
        // Note: If runEvent returns true, return null
        // Note: Otherwise return the modified locked move ID

        // Currently just returns locked_move field directly
        self.locked_move.as_ref()
    }
}

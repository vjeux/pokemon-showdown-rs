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

use crate::*;

impl Pokemon {
    /// Get locked move (for multi-turn moves)
    /// Equivalent to pokemon.ts getLockedMove()
    ///
    /// This refers to multi-turn moves like SolarBeam and Outrage and
    /// Sky Drop, which remove all choice (no dynamax, switching, etc).
    /// Don't use it for "soft locks" like Choice Band.
    ///
    /// This is an associated function (not a method) because it needs
    /// access to Battle for the runEvent() call.
    /// Call as: Pokemon::get_locked_move(battle, pokemon_pos)
    pub fn get_locked_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> Option<ID> {
        // Phase 1: Get locked_move field immutably
        let locked_move = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return None,
            };
            pokemon.locked_move.clone()
        };

        // JS: const lockedMove = this.battle.runEvent('LockMove', this);
        // ✅ NOW IMPLEMENTED: battle.run_event('LockMove') call
        let event_result = battle.run_event("LockMove", Some(pokemon_pos), None, None, None);

        // JS: return (lockedMove === true) ? null : lockedMove;
        // ✅ NOW IMPLEMENTED: If runEvent returns Some(1) (true), return None
        // ✅ Otherwise return the locked_move from the field
        match event_result {
            Some(1) => None, // true in JavaScript → null
            _ => locked_move, // Otherwise use the locked_move field
        }
    }
}

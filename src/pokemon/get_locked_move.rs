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
use crate::event::EventResult;

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
        // JS: const lockedMove = this.battle.runEvent('LockMove', this);
        // ✅ NOW IMPLEMENTED: battle.run_event('LockMove') call
        // The event handlers return the locked move ID from volatile effectState
        let event_result = battle.run_event("LockMove", Some(crate::event::EventTarget::Pokemon(pokemon_pos)), None, None, EventResult::Continue, false, false);

        // JS: return (lockedMove === true) ? null : lockedMove;
        // ✅ NOW IMPLEMENTED: If runEvent returns true, return None
        // ✅ Otherwise return the move ID from the event result
        match event_result {
            EventResult::Boolean(true) => None, // true in JavaScript → null
            EventResult::String(move_id) => Some(ID::from(move_id)), // Event handler returned move ID as string
            EventResult::ID(move_id) => Some(move_id), // Event handler returned move ID directly
            EventResult::Continue => None, // No event handler fired, no locked move
            _ => None, // Other results treated as no lock
        }
    }
}

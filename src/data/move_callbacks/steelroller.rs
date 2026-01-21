//! Steel Roller Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTry() {
///     return !this.field.isTerrain('');
/// }
pub fn on_try(
    battle: &mut Battle,
    _source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // return !this.field.isTerrain('');
    // Returns true if there is a terrain active (not empty)
    // Returns false if no terrain (move fails)
    let has_terrain = !battle.is_terrain("");

    if !has_terrain {
        // No terrain - move fails (return false, NOT NotFail)
        // NotFail ('') means "succeeded but did nothing" (like Future Sight queuing)
        // Boolean(false) means "move failed" and shows -fail message
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}

/// onHit() {
///     this.field.clearTerrain();
/// }
pub fn on_hit(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // this.field.clearTerrain();
    battle.clear_terrain();

    EventResult::Continue
}

/// onAfterSubDamage() {
///     this.field.clearTerrain();
/// }
pub fn on_after_sub_damage(battle: &mut Battle) -> EventResult {
    // this.field.clearTerrain();
    battle.clear_terrain();

    EventResult::Continue
}

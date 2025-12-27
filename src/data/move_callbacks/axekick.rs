//! Axe Kick Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onMoveFail(target, source, move) {
///     this.damage(source.baseMaxhp / 2, source, source, this.dex.conditions.get('High Jump Kick'));
/// }
pub fn on_move_fail(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    let source_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Get source's base max HP for crash damage calculation
    let base_maxhp = if let Some(source) = battle.pokemon_at(source_pos.0, source_pos.1) {
        source.base_maxhp
    } else {
        return EventResult::Continue;
    };

    // Deal crash damage: half of base max HP
    let crash_damage = base_maxhp / 2;
    battle.damage(crash_damage, source_pos, Some(source_pos), Some("High Jump Kick"));

    EventResult::Continue
}


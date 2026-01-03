//! Super Fang Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// damageCallback(pokemon, target) {
///     return this.clampIntRange(target.getUndynamaxedHP() / 2, 1);
/// }
pub fn damage_callback(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    // Get the target
    let target = match target_pos {
        Some(pos) => match battle.pokemon_at(pos.0, pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        },
        None => return EventResult::Continue,
    };

    // return this.clampIntRange(target.getUndynamaxedHP() / 2, 1);
    let damage = (target.get_undynamaxed_hp(None) / 2).max(1);

    EventResult::Number(damage)
}

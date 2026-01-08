//! Grassy Glide Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyPriority(priority, source, target, move) {
///     if (this.field.isTerrain('grassyterrain') && source.isGrounded()) {
///         return priority + 1;
///     }
/// }
pub fn on_modify_priority(
    battle: &mut Battle,
    source_pos: Option<(usize, usize)>,
    _target_pos: Option<(usize, usize)>,
    _active_move: Option<&crate::battle_actions::ActiveMove>,
) -> EventResult {
    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (this.field.isTerrain('grassyterrain') && source.isGrounded()) {
    let is_grassy_terrain = battle.is_terrain("grassyterrain");

    let is_grounded = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.is_grounded(battle, false).unwrap_or(false)
    };

    if is_grassy_terrain && is_grounded {
        // return priority + 1;
        return EventResult::Number(1);
    }

    EventResult::Continue
}

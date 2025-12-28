//! Ice Spinner Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAfterHit(target, source) {
///     if (source.hp) {
///         this.field.clearTerrain();
///     }
/// }
pub fn on_after_hit(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: (usize, usize),
) -> EventResult {
    let source = source_pos;

    // if (source.hp) {
    //     this.field.clearTerrain();
    // }
    let source_has_hp = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.hp > 0
    };

    if source_has_hp {
        battle.field.clear_terrain();
    }

    EventResult::Continue
}

/// onAfterSubDamage(damage, target, source) {
///     if (source.hp) {
///         this.field.clearTerrain();
///     }
/// }
pub fn on_after_sub_damage(
    battle: &mut Battle,
    _damage: i32,
    _target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (source.hp) {
    //     this.field.clearTerrain();
    // }
    let source_has_hp = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.hp > 0
    };

    if source_has_hp {
        battle.field.clear_terrain();
    }

    EventResult::Continue
}

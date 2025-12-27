//! Hold Back Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamage(damage, target, source, effect) {
///     if (damage >= target.hp) return target.hp - 1;
/// }
pub fn on_damage(battle: &mut Battle, damage: i32, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    let target = target_pos;

    // if (damage >= target.hp) return target.hp - 1;
    let target_hp = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.hp
    };

    if damage >= target_hp {
        return EventResult::Int(target_hp - 1);
    }

    EventResult::Continue
}


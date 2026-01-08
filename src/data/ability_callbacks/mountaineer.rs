//! Mountaineer Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamage(damage, target, source, effect) {
///     if (effect && effect.id === 'stealthrock') {
///         return false;
///     }
/// }
pub fn on_damage(_battle: &mut Battle, _damage: i32, _target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    if let Some(effect) = effect_id {
        if effect == "stealthrock" {
            return EventResult::Boolean(false);
        }
    }
    EventResult::Continue
}

/// onTryHit(target, source, move) {
///     if (move.type === 'Rock' && !target.activeTurns) {
///         this.add('-immune', target, '[from] ability: Mountaineer');
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), _source_pos: (usize, usize), _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    use crate::battle::Arg;

    // if (move.type === 'Rock' && !target.activeTurns)
    let move_type = if let Some(ref active_move) = battle.active_move {
        active_move.move_type.clone()
    } else {
        return EventResult::Continue;
    };

    if move_type != "Rock" {
        return EventResult::Continue;
    }

    let active_turns = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.active_turns
    };

    if active_turns != 0 {
        return EventResult::Continue;
    }

    // this.add('-immune', target, '[from] ability: Mountaineer');
    let target_id = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.get_slot()
    };

    battle.add("-immune", &[
        Arg::String(target_id),
        Arg::Str("[from] ability: Mountaineer"),
    ]);

    // return null;
    EventResult::Null
}


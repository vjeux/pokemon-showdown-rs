//! Battle Bond Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSourceAfterFaint(length, target, source, effect) {
///     if (source.bondTriggered) return;
///     if (effect?.effectType !== 'Move') return;
///     if (source.species.id === 'greninjabond' && source.hp && !source.transformed && source.side.foePokemonLeft()) {
///         this.boost({ atk: 1, spa: 1, spe: 1 }, source, source, this.effect);
///         this.add('-activate', source, 'ability: Battle Bond');
///         source.bondTriggered = true;
///     }
/// }
pub fn on_source_after_faint(_battle: &mut Battle, _target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onModifyMove(move, attacker) {
///     if (move.id === 'watershuriken' && attacker.species.name === 'Greninja-Ash' &&
///         !attacker.transformed) {
///         move.multihit = 3;
///     }
/// }
pub fn on_modify_move(battle: &mut Battle, _move_id: &str) -> EventResult {
    // Get attacker position from current event
    let attacker_pos = match &battle.current_event {
        Some(event) => match event.source {
            Some(pos) => pos,
            None => return EventResult::Continue,
        },
        None => return EventResult::Continue,
    };

    // Check if move is watershuriken
    let is_water_shuriken = if let Some(ref active_move) = battle.active_move {
        active_move.id.as_str() == "watershuriken"
    } else {
        return EventResult::Continue;
    };

    if !is_water_shuriken {
        return EventResult::Continue;
    }

    // if (attacker.species.name === 'Greninja-Ash' && !attacker.transformed)
    let (species_name, transformed) = {
        let attacker = match battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let species_data = match battle.dex.species().get(attacker.species_id.as_str()) {
            Some(data) => data,
            None => return EventResult::Continue,
        };

        (species_data.name.clone(), attacker.transformed)
    };

    if species_name == "Greninja-Ash" && !transformed {
        // move.multihit = 3;
        if let Some(ref mut active_move) = battle.active_move {
            active_move.multi_hit = Some(3);
        }
    }

    EventResult::Continue
}


//! Thermal Exchange Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (move.type === 'Fire') {
///         this.boost({ atk: 1 });
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult { let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (move.type === 'Fire')
    if let Some(move_data) = battle.dex.moves().get(move_id) {
        if move_data.move_type == "Fire" {
            // this.boost({ atk: 1 });
            battle.boost(
                &[("atk", 1)],
                target_pos,
                Some(target_pos),
                None,
                false,
                false,
            );
        }
    }

    EventResult::Continue
}

/// onUpdate(pokemon) {
///     if (pokemon.status === 'brn') {
///         this.add('-activate', pokemon, 'ability: Thermal Exchange');
///         pokemon.cureStatus();
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::battle::Arg;
    use crate::Pokemon;

    // if (pokemon.status === 'brn')
    let has_burn = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.status.as_str() == "brn"
    };

    if has_burn {
        // this.add('-activate', pokemon, 'ability: Thermal Exchange');
        let pokemon_slot = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };

        battle.add("-activate", &[
            Arg::String(pokemon_slot),
            Arg::Str("ability: Thermal Exchange"),
        ]);

        // pokemon.cureStatus();
        Pokemon::cure_status(battle, pokemon_pos, false);
    }

    EventResult::Continue
}

/// onSetStatus(status, target, source, effect) {
///     if (status.id !== 'brn') return;
///     if ((effect as Move)?.status) {
///         this.add('-immune', target, '[from] ability: Thermal Exchange');
///     }
///     return false;
/// }
pub fn on_set_status(battle: &mut Battle, status_id: &str, target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    use crate::battle::Arg;

    // if (status.id !== 'brn') return;
    if status_id != "brn" {
        return EventResult::Continue;
    }

    // if ((effect as Move)?.status)
    // Check if the effect is a move by looking it up in the moves dex
    let is_move_with_status = if let Some(effect) = effect_id {
        if let Some(move_data) = battle.dex.moves().get(effect) {
            // If it's a move, check if it has a status effect
            move_data.status.is_some()
        } else {
            false
        }
    } else {
        false
    };

    if is_move_with_status {
        // this.add('-immune', target, '[from] ability: Thermal Exchange');
        let target_slot = {
            let pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Boolean(false),
            };
            pokemon.get_slot()
        };

        battle.add("-immune", &[
            Arg::String(target_slot),
            Arg::Str("[from] ability: Thermal Exchange"),
        ]);
    }

    // return false;
    EventResult::Boolean(false)
}


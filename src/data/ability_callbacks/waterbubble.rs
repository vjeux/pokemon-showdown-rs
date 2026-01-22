//! Water Bubble Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::{Battle, Arg, Effect};
use crate::event::EventResult;

/// onSourceModifyAtk(atk, attacker, defender, move) {
///     if (move.type === 'Fire') {
///         return this.chainModify(0.5);
///     }
/// }
pub fn on_source_modify_atk(battle: &mut Battle, _atk: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // JavaScript checks move.type (the active move's type, not the dex type)
    // This is important for moves whose type is modified at runtime (e.g., Electrify)
    if active_move.map(|m| m.move_type == "Fire").unwrap_or(false) {
        battle.chain_modify(0.5);
    }
    EventResult::Continue
}

/// onSourceModifySpA(atk, attacker, defender, move) {
///     if (move.type === 'Fire') {
///         return this.chainModify(0.5);
///     }
/// }
pub fn on_source_modify_sp_a(battle: &mut Battle, _spa: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // JavaScript checks move.type (the active move's type, not the dex type)
    if active_move.map(|m| m.move_type == "Fire").unwrap_or(false) {
        battle.chain_modify(0.5);
    }
    EventResult::Continue
}

/// onModifyAtk(atk, attacker, defender, move) {
///     if (move.type === 'Water') {
///         return this.chainModify(2);
///     }
/// }
pub fn on_modify_atk(battle: &mut Battle, _atk: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // JavaScript checks move.type (the active move's type, not the dex type)
    if active_move.map(|m| m.move_type == "Water").unwrap_or(false) {
        battle.chain_modify(2.0);
    }
    EventResult::Continue
}

/// onModifySpA(atk, attacker, defender, move) {
///     if (move.type === 'Water') {
///         return this.chainModify(2);
///     }
/// }
pub fn on_modify_sp_a(battle: &mut Battle, _spa: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // JavaScript checks move.type (the active move's type, not the dex type)
    if active_move.map(|m| m.move_type == "Water").unwrap_or(false) {
        battle.chain_modify(2.0);
    }
    EventResult::Continue
}

/// onUpdate(pokemon) {
///     if (pokemon.status === 'brn') {
///         this.add('-activate', pokemon, 'ability: Water Bubble');
///         pokemon.cureStatus();
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.status === 'brn') {
    //     this.add('-activate', pokemon, 'ability: Water Bubble');
    //     pokemon.cureStatus();
    // }

    // Check if Pokemon has burn status
    let has_burn = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.status.as_str() == "brn"
    };

    if has_burn {
        // Get Pokemon ID for the message
        let pokemon_id = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            let side_id = format!("p{}", pokemon.side_index + 1);
            if pokemon.is_active {
                let pos_letter = (b'a' + pokemon.position as u8) as char;
                format!("{}{}: {}", side_id, pos_letter, pokemon.name)
            } else {
                format!("{}: {}", side_id, pokemon.name)
            }
        };

        // Show activation message
        battle.add("-activate", &[
            Arg::String(pokemon_id),
            Arg::Str("ability: Water Bubble"),
        ]);

        // Cure the status
        use crate::pokemon::Pokemon;
        Pokemon::cure_status(battle, pokemon_pos, false);
    }

    EventResult::Continue
}

/// onSetStatus(status, target, source, effect) {
///     if (status.id !== 'brn') return;
///     if ((effect as Move)?.status) {
///         this.add('-immune', target, '[from] ability: Water Bubble');
///     }
///     return false;
/// }
pub fn on_set_status(battle: &mut Battle, status_id: &str, target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, effect: Option<&Effect>) -> EventResult {
    let effect_id = effect.map(|e| e.id.as_str());
    // if (status.id !== 'brn') return;
    if status_id != "brn" {
        return EventResult::Continue;
    }

    // if ((effect as Move)?.status)
    // Check if effect is a move with a status secondary
    if let Some(effect) = effect_id {
        if let Some(move_data) = battle.dex.moves().get(effect) {
            // If move has status secondary (like Scald has burn), show immune message
            if move_data.status.is_some() || move_data.secondary.iter().any(|sec| sec.status.is_some()) {
                // Get Pokemon ID for the message
                let pokemon_id = {
                    let pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                        Some(p) => p,
                        None => return EventResult::Boolean(false),
                    };

                    let side_id = format!("p{}", pokemon.side_index + 1);
                    if pokemon.is_active {
                        let pos_letter = (b'a' + pokemon.position as u8) as char;
                        format!("{}{}: {}", side_id, pos_letter, pokemon.name)
                    } else {
                        format!("{}: {}", side_id, pokemon.name)
                    }
                };

                battle.add("-immune", &[
                    Arg::String(pokemon_id),
                    Arg::Str("[from] ability: Water Bubble"),
                ]);
            }
        }
    }

    // return false;
    EventResult::Boolean(false)
}


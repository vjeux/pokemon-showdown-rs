//! Vital Spirit Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onUpdate(pokemon) {
///     if (pokemon.status === 'slp') {
///         this.add('-activate', pokemon, 'ability: Vital Spirit');
///         pokemon.cureStatus();
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    let (is_asleep, pokemon_ident, pokemon_name) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon.status == "slp".into(), pokemon.get_slot(), pokemon.name.clone())
    };

    if is_asleep {
        battle.add(
            "-activate",
            &[
                pokemon_ident.as_str().into(),
                "ability: Vital Spirit".into(),
            ],
        );

        // pokemon.cureStatus()
        if let Some((status, removed_nightmare, _silent)) = Pokemon::cure_status(battle, pokemon_pos, false) {
            let full_name = format!("{}: {}", pokemon_ident, pokemon_name);
            battle.add("-curestatus", &[full_name.as_str().into(), status.as_str().into(), "[msg]".into()]);
            if removed_nightmare {
                battle.add("-end", &[full_name.as_str().into(), "Nightmare".into(), "[silent]".into()]);
            }
        }
    }

    EventResult::Continue
}

/// onSetStatus(status, target, source, effect) {
///     if (status.id !== 'slp') return;
///     if ((effect as Move)?.status) {
///         this.add('-immune', target, '[from] ability: Vital Spirit');
///     }
///     return false;
/// }
pub fn on_set_status(battle: &mut Battle, status_id: &str, target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    if status_id != "slp" {
        return EventResult::Continue;
    }

    // Check if effect is a move with status
    if let Some(eff_id) = effect_id {
        if let Some(move_data) = battle.dex.moves().get(eff_id) {
            if move_data.status.is_some() {
                let target_ident = {
                    let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                        Some(p) => p,
                        None => return EventResult::Boolean(false),
                    };
                    target.get_slot()
                };

                battle.add(
                    "-immune",
                    &[
                        target_ident.as_str().into(),
                        "[from] ability: Vital Spirit".into(),
                    ],
                );
            }
        }
    }

    EventResult::Boolean(false)
}

/// onTryAddVolatile(status, target) {
///     if (status.id === 'yawn') {
///         this.add('-immune', target, '[from] ability: Vital Spirit');
///         return null;
///     }
/// }
pub fn on_try_add_volatile(battle: &mut Battle, status_id: &str, target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    if status_id == "yawn" {
        let target_ident = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Null,
            };
            target.get_slot()
        };

        battle.add(
            "-immune",
            &[
                target_ident.as_str().into(),
                "[from] ability: Vital Spirit".into(),
            ],
        );
        return EventResult::Null;
    }
    EventResult::Continue
}


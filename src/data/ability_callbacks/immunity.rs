//! Immunity Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onUpdate(pokemon) {
///     if (pokemon.status === 'psn' || pokemon.status === 'tox') {
///         this.add('-activate', pokemon, 'ability: Immunity');
///         pokemon.cureStatus();
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    let (is_poisoned, pokemon_ident, pokemon_name) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let poisoned = pokemon.status == "psn".into() || pokemon.status == "tox".into();
        (poisoned, pokemon.get_slot(), pokemon.name.clone())
    };

    if is_poisoned {
        battle.add(
            "-activate",
            &[
                pokemon_ident.as_str().into(),
                "ability: Immunity".into(),
            ],
        );

        // pokemon.cureStatus()
        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        if let Some((status, removed_nightmare)) = pokemon_mut.cure_status() {
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
///     if (status.id !== 'psn' && status.id !== 'tox') return;
///     if ((effect as Move)?.status) {
///         this.add('-immune', target, '[from] ability: Immunity');
///     }
///     return false;
/// }
pub fn on_set_status(battle: &mut Battle, status_id: &str, target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    if status_id != "psn" && status_id != "tox" {
        return EventResult::Continue;
    }

    // Check if effect is a move with status
    if let Some(eff_id) = effect_id {
        if let Some(move_data) = battle.dex.get_move(eff_id) {
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
                        "[from] ability: Immunity".into(),
                    ],
                );
            }
        }
    }

    EventResult::Boolean(false)
}


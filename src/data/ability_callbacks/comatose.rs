//! Comatose Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     this.add('-ability', pokemon, 'Comatose');
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    let pokemon_ident = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add(
        "-ability",
        &[
            pokemon_ident.as_str().into(),
            "Comatose".into(),
        ],
    );
    EventResult::Continue
}

/// onSetStatus(status, target, source, effect) {
///     if ((effect as Move)?.status) {
///         this.add('-immune', target, '[from] ability: Comatose');
///     }
///     return false;
/// }
pub fn on_set_status(battle: &mut Battle, _status_id: &str, target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
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
                        "[from] ability: Comatose".into(),
                    ],
                );
            }
        }
    }

    EventResult::Boolean(false)
}


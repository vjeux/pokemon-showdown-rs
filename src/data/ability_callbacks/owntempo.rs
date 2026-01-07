//! Own Tempo Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onUpdate(pokemon) {
///     if (pokemon.volatiles['confusion']) {
///         this.add('-activate', pokemon, 'ability: Own Tempo');
///         pokemon.removeVolatile('confusion');
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::battle::Arg;
    use crate::Pokemon;

    // if (pokemon.volatiles['confusion'])
    let has_confusion = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.volatiles.contains_key(&crate::dex_data::ID::from("confusion"))
    };

    if has_confusion {
        // this.add('-activate', pokemon, 'ability: Own Tempo');
        let pokemon_slot = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };

        battle.add("-activate", &[
            Arg::String(pokemon_slot),
            Arg::Str("ability: Own Tempo"),
        ]);

        // pokemon.removeVolatile('confusion');
        Pokemon::remove_volatile(battle, pokemon_pos, &crate::dex_data::ID::from("confusion"));
    }

    EventResult::Continue
}

/// onTryAddVolatile(status, pokemon) {
///     if (status.id === 'confusion') return null;
/// }
pub fn on_try_add_volatile(_battle: &mut Battle, status_id: &str, _target_pos: (usize, usize), __source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    if status_id == "confusion" {
        return EventResult::Null;
    }
    EventResult::Continue
}

/// onHit(target, source, move) {
///     if (move?.volatileStatus === 'confusion') {
///         this.add('-immune', target, 'confusion', '[from] ability: Own Tempo');
///     }
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: (usize, usize), move_id: &str) -> EventResult {
    if let Some(move_data) = battle.dex.moves().get(move_id) {
        if let Some(ref volatile_status) = move_data.volatile_status {
            if volatile_status.as_str() == "confusion" {
                let target_ident = {
                    let target = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    target.get_slot()
                };

                battle.add(
                    "-immune",
                    &[
                        target_ident.as_str().into(),
                        "confusion".into(),
                        "[from] ability: Own Tempo".into(),
                    ],
                );
            }
        }
    }
    EventResult::Continue
}

/// onTryBoost(boost, target, source, effect) {
///     if (effect.name === 'Intimidate' && boost.atk) {
///         delete boost.atk;
///         this.add('-fail', target, 'unboost', 'Attack', '[from] ability: Own Tempo', `[of] ${target}`);
///     }
/// }
pub fn on_try_boost(
    battle: &mut Battle,
    boost: Option<&mut crate::dex_data::BoostsTable>, target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>,
) -> EventResult {
    let is_intimidate = battle.current_event.as_ref()
        .and_then(|e| e.effect.as_ref())
        .map(|id| id.id.as_str() == "intimidate")
        .unwrap_or(false);

    if !is_intimidate {
        return EventResult::Continue;
    }

    // Check if we have a boost table
    let boost = match boost {
        Some(b) => b,
        None => return EventResult::Continue,
    };

    // if (boost.atk) {
    if boost.atk != 0 {
        // delete boost.atk;
        boost.atk = 0;

        let target_slot = {
            let pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };

        battle.add(
            "-fail",
            &[
                crate::battle::Arg::from(target_slot.clone()),
                crate::battle::Arg::from("unboost"),
                crate::battle::Arg::from("Attack"),
                crate::battle::Arg::from("[from] ability: Own Tempo"),
                crate::battle::Arg::from(format!("[of] {}", target_slot)),
            ],
        );
    }

    EventResult::Continue
}


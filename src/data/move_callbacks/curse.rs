//! Curse Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, hp_fraction};
use crate::event::EventResult;

/// onModifyMove(move, source, target) {
///     if (!source.hasType('Ghost')) {
///         move.target = move.nonGhostTarget!;
///     } else if (source.isAlly(target)) {
///         move.target = 'randomNormal';
///     }
/// }
pub fn on_modify_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    // if (!source.hasType('Ghost')) {
    //     move.target = move.nonGhostTarget!;
    // } else if (source.isAlly(target)) {
    //     move.target = 'randomNormal';
    // }
    let source = pokemon_pos;

    let source_has_ghost = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.has_type(battle, "ghost")
    };

    if !source_has_ghost {
        // move.target = move.nonGhostTarget!;
        // Modify the active move's target to nonGhostTarget
        // In the JavaScript data, Curse has nonGhostTarget: "self"
        if let Some(ref mut active_move) = battle.active_move {
            // The nonGhostTarget for Curse is "self" (targets the user, not an opponent)
            active_move.borrow_mut().target = "self".to_string();
        }
    } else if let Some(target) = target_pos {
        // else if (source.isAlly(target)) {
        //     move.target = 'randomNormal';
        // }
        let is_ally = battle.is_ally(source, target);
        if is_ally {
            // move.target = 'randomNormal';
            if let Some(ref mut active_move) = battle.active_move {
                active_move.borrow_mut().target = "randomNormal".to_string();
            }
        }
    }

    EventResult::Continue
}

/// onTryHit(target, source, move) {
///     if (!source.hasType('Ghost')) {
///         delete move.volatileStatus;
///         delete move.onHit;
///         move.self = { boosts: { spe: -1, atk: 1, def: 1 } };
///     } else if (move.volatileStatus && target.volatiles['curse']) {
///         return false;
///     }
/// }
pub fn on_try_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),
    source_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;

    // JavaScript: onTryHit(target, source, move) - target comes first, source second
    let source = source_pos;
    let target = target_pos;

    let source_has_ghost = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.has_type(battle, "ghost")
    };

    if !source_has_ghost {
        // delete move.volatileStatus;
        // delete move.onHit;
        // move.self = { boosts: { spe: -1, atk: 1, def: 1 } };
        // Modify the active move to be the non-Ghost version
        if let Some(ref mut active_move) = battle.active_move {
            // Clear volatile status (delete move.volatileStatus)
            active_move.borrow_mut().volatile_status = None;

            // Set self-boost effect (move.self = { boosts: { spe: -1, atk: 1, def: 1 } })
            use crate::dex::MoveSecondary;
            use crate::dex_data::BoostsTable;

            active_move.borrow_mut().self_effect = Some(MoveSecondary {
                boosts: Some(BoostsTable {
                    atk: 1,
                    def: 1,
                    spa: 0,
                    spd: 0,
                    spe: -1,
                    accuracy: 0,
                    evasion: 0,
                }),
                ..Default::default()
            });
        }
    } else {
        // else if (move.volatileStatus && target.volatiles['curse']) {
        //     return false;
        // }
        let has_volatile_status = battle.active_move.as_ref()
            .map(|m| m.borrow().volatile_status.is_some())
            .unwrap_or(false);

        if has_volatile_status {
            let has_curse_volatile = {
                let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                target_pokemon.volatiles.contains_key(&ID::from("curse"))
            };

            if has_curse_volatile {
                return EventResult::Boolean(false);
            }
        }
    }

    EventResult::Continue
}

/// onHit(target, source) {
///     this.directDamage(source.maxhp / 2, source, source);
/// }
/// NOTE: This callback is only called for Ghost-type users of Curse.
/// For non-Ghost users, JavaScript does `delete move.onHit` in onTryHit.
/// We check Ghost type here since Rust dispatch is static.
pub fn on_hit(
    battle: &mut Battle,
    _target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    // JavaScript: onHit(target, source) - but only for Ghost-type users
    // For non-Ghost, onTryHit does: delete move.onHit
    // We simulate this by checking Ghost type here
    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let source_has_ghost = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.has_type(battle, "ghost")
    };

    // Only Ghost-type users perform this onHit behavior (directDamage)
    // Non-Ghost users have `delete move.onHit` in JavaScript's onTryHit
    if !source_has_ghost {
        return EventResult::Continue;
    }

    // this.directDamage(source.maxhp / 2, source, source);
    // NOTE: JavaScript uses floating-point division, so 1/2 = 0.5, which is truthy.
    // directDamage then clamps to at least 1. Use hp_fraction for the same behavior.
    let damage = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        hp_fraction(source_pokemon.maxhp, 2)
    };

    battle.direct_damage(damage, Some(source), Some(source), None);

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(pokemon, source) {
    ///     this.add('-start', pokemon, 'Curse', `[of] ${source}`);
    /// }
    pub fn on_start(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        source_pos: Option<(usize, usize)>,
        _effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
        // this.add('-start', pokemon, 'Curse', `[of] ${source}`);
        let pokemon = pokemon_pos;
        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        let (pokemon_ident, source_ident) = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            (pokemon_pokemon.get_slot(), source_pokemon.get_slot())
        };

        battle.add(
            "-start",
            &[
                pokemon_ident.as_str().into(),
                "Curse".into(),
                format!("[of] {}", source_ident).into(),
            ],
        );

        EventResult::Continue
    }

    /// onResidual(pokemon) {
    ///     this.damage(pokemon.baseMaxhp / 4);
    /// }
    pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // this.damage(pokemon.baseMaxhp / 4);
        let pokemon = pokemon_pos;

        let damage = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            hp_fraction(pokemon_pokemon.base_maxhp, 4)
        };

        battle.damage(damage, Some(pokemon), None, None, false);

        EventResult::Continue
    }
}

//! BattleActions::hitStepTryImmunity - Check for immunity to moves
//!
//! 1:1 port of hitStepTryImmunity from battle-actions.ts

use crate::*;
use crate::battle_actions::ActiveMove;
use crate::event::EventResult;

/// Check for immunity (powder moves, prankster, etc.)
/// Equivalent to battle-actions.ts hitStepTryImmunity()
///
/// hitStepTryImmunity(targets: Pokemon[], pokemon: Pokemon, move: ActiveMove) {
///     const hitResults = [];
///     for (const [i, target] of targets.entries()) {
///         if (this.battle.gen >= 6 && move.flags['powder'] && target !== pokemon && !this.dex.getImmunity('powder', target)) {
///             this.battle.debug('natural powder immunity');
///             this.battle.add('-immune', target);
///             hitResults[i] = false;
///         } else if (!this.battle.singleEvent('TryImmunity', move, {}, target, pokemon, move)) {
///             this.battle.add('-immune', target);
///             hitResults[i] = false;
///         } else if (this.battle.gen >= 7 && move.pranksterBoosted && pokemon.hasAbility('prankster') &&
///             !targets[i].isAlly(pokemon) && !this.dex.getImmunity('prankster', target)) {
///             this.battle.debug('natural prankster immunity');
///             if (target.illusion || !(move.status && !this.dex.getImmunity(move.status, target))) {
///                 this.battle.hint("Since gen 7, Dark is immune to Prankster moves.");
///             }
///             this.battle.add('-immune', target);
///             hitResults[i] = false;
///         } else {
///             hitResults[i] = true;
///         }
///     }
///     return hitResults;
/// }
pub fn hit_step_try_immunity(
    battle: &mut Battle,
    targets: &[(usize, usize)],
    attacker_pos: (usize, usize),
    active_move: &ActiveMove,
) -> Vec<bool> {
    // const hitResults = [];
    let mut hit_results = Vec::new();

    // for (const [i, target] of targets.entries()) {
    for &target_pos in targets {
        let hit_result = {
            // if (this.battle.gen >= 6 && move.flags['powder'] && target !== pokemon && !this.dex.getImmunity('powder', target)) {
            //     this.battle.debug('natural powder immunity');
            //     this.battle.add('-immune', target);
            //     hitResults[i] = false;
            if battle.gen >= 6 && active_move.flags.powder && target_pos != attacker_pos {
                let has_powder_immunity = {
                    let target_pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                        Some(p) => p,
                        None => return vec![false; targets.len()],
                    };
                    battle.dex.get_immunity("powder", &target_pokemon.types)
                };
                if !has_powder_immunity {
                    battle.debug("natural powder immunity");
                    if let Some(target_pokemon) = battle.pokemon_at(target_pos.0, target_pos.1) {
                        let target_ident = format!("p{}a: {}", target_pos.0 + 1, target_pokemon.set.species);
                        battle.add("-immune", &[
                            crate::battle::Arg::String(target_ident),
                        ]);
                    }
                    false
                } else {
                    // Check TryImmunity event
                    self_check_try_immunity(battle, target_pos, attacker_pos, active_move)
                }
            } else {
                // Check TryImmunity event
                self_check_try_immunity(battle, target_pos, attacker_pos, active_move)
            }
        };

        hit_results.push(hit_result);
    }

    // return hitResults;
    hit_results
}

fn self_check_try_immunity(
    battle: &mut Battle,
    target_pos: (usize, usize),
    attacker_pos: (usize, usize),
    active_move: &ActiveMove,
) -> bool {
    // } else if (!this.battle.singleEvent('TryImmunity', move, {}, target, pokemon, move)) {
    //     this.battle.add('-immune', target);
    //     hitResults[i] = false;
    let try_immunity_result = battle.single_event(
        "TryImmunity",
        &crate::battle::Effect::move_(active_move.id.clone()),
        Some(target_pos),
        Some(attacker_pos),
        None,
        None,
    );

    if matches!(try_immunity_result, EventResult::Boolean(false)) {
        // Event returned false
        if let Some(target_pokemon) = battle.pokemon_at(target_pos.0, target_pos.1) {
            let target_ident = format!("p{}a: {}", target_pos.0 + 1, target_pokemon.set.species);
            battle.add("-immune", &[
                crate::battle::Arg::String(target_ident),
            ]);
        }
        return false;
    }

    // } else if (this.battle.gen >= 7 && move.pranksterBoosted && pokemon.hasAbility('prankster') &&
    //     !targets[i].isAlly(pokemon) && !this.dex.getImmunity('prankster', target)) {
    if battle.gen >= 7 && active_move.prankster_boosted {
        let attacker_has_prankster = battle.pokemon_at(attacker_pos.0, attacker_pos.1)
            .map(|p| p.has_ability(battle, &["prankster"]))
            .unwrap_or(false);

        if attacker_has_prankster {
            let is_ally = target_pos.0 == attacker_pos.0; // Same side = ally
            if !is_ally {
                let has_prankster_immunity = {
                    let target_pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                        Some(p) => p,
                        None => return true,
                    };
                    battle.dex.get_immunity("prankster", &target_pokemon.types)
                };
                if !has_prankster_immunity {
                    //     this.battle.debug('natural prankster immunity');
                    battle.debug("natural prankster immunity");

                    //     if (target.illusion || !(move.status && !this.dex.getImmunity(move.status, target))) {
                    //         this.battle.hint("Since gen 7, Dark is immune to Prankster moves.");
                    //     }
                    let target_illusion = battle.pokemon_at(target_pos.0, target_pos.1)
                        .map(|p| p.illusion.is_some())
                        .unwrap_or(false);

                    let show_hint = if target_illusion {
                        true
                    } else if let Some(status) = &active_move.status {
                        let status_immunity = {
                            let target_pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                                Some(p) => p,
                                None => return true,
                            };
                            battle.dex.get_immunity(status.as_str(), &target_pokemon.types)
                        };
                        // !(move.status && !this.dex.getImmunity(move.status, target))
                        // = !(!status_immunity)
                        // = status_immunity
                        status_immunity
                    } else {
                        true
                    };

                    if show_hint {
                        battle.hint("Since gen 7, Dark is immune to Prankster moves.", false, None);
                    }

                    //     this.battle.add('-immune', target);
                    //     hitResults[i] = false;
                    if let Some(target_pokemon) = battle.pokemon_at(target_pos.0, target_pos.1) {
                        let target_ident = format!("p{}a: {}", target_pos.0 + 1, target_pokemon.set.species);
                        battle.add("-immune", &[
                            crate::battle::Arg::String(target_ident),
                        ]);
                    }
                    return false;
                }
            }
        }
    }

    // } else {
    //     hitResults[i] = true;
    // }
    true
}

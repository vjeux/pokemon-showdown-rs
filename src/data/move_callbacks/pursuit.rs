//! Pursuit Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::Pokemon;

/// basePowerCallback(pokemon, target, move) {
///     // You can't get here unless the pursuit succeeds
///     if (target.beingCalledBack || target.switchFlag) {
///         this.debug('Pursuit damage boost');
///         return move.basePower * 2;
///     }
///     return move.basePower;
/// }
pub fn base_power_callback(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    // Get the target
    let target = match target_pos {
        Some(pos) => match battle.pokemon_at(pos.0, pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        },
        None => return EventResult::Continue,
    };

    // Get the active move
    let active_move = match &battle.active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // You can't get here unless the pursuit succeeds
    // if (target.beingCalledBack || target.switchFlag)
    if target.being_called_back || target.switch_flag.is_some() {
        let bp = active_move.base_power * 2;
        // this.debug('Pursuit damage boost');
        battle.debug("Pursuit damage boost");
        return EventResult::Number(bp);
    }

    EventResult::Number(active_move.base_power)
}

/// beforeTurnCallback(pokemon) {
///     for (const side of this.sides) {
///         if (side.hasAlly(pokemon)) continue;
///         side.addSideCondition('pursuit', pokemon);
///         const data = side.getSideConditionData('pursuit');
///         if (!data.sources) {
///             data.sources = [];
///         }
///         data.sources.push(pokemon);
///     }
/// }
pub fn before_turn_callback(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    let pokemon = pokemon_pos;

    // Validate pokemon exists
    if battle.pokemon_at(pokemon.0, pokemon.1).is_none() {
        return EventResult::Continue;
    }

    // for (const side of this.sides) {
    for side_idx in 0..battle.sides.len() {
        // if (side.hasAlly(pokemon)) continue;
        let is_ally = pokemon.0 == side_idx || battle.sides[side_idx].ally_index == Some(pokemon.0);
        if is_ally {
            continue;
        }

        // side.addSideCondition('pursuit', pokemon);
        let pursuit_id = ID::from("pursuit");
        battle.add_side_condition(side_idx, pursuit_id.clone(), Some(pokemon), None);

        // const data = side.getSideConditionData('pursuit');
        // if (!data.sources) {
        //     data.sources = [];
        // }
        // data.sources.push(pokemon);
        if let Some(state) = battle.get_side_condition_data_mut(side_idx, &pursuit_id) {
            // Get or create sources array and add pokemon position
            let sources = state.sources.get_or_insert_with(Vec::new);
            sources.push(pokemon);
        }
    }

    EventResult::Continue
}

/// onModifyMove(move, source, target) {
///     if (target?.beingCalledBack || target?.switchFlag) move.accuracy = true;
/// }
pub fn on_modify_move(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (target?.beingCalledBack || target?.switchFlag) move.accuracy = true;
    let should_always_hit = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.being_called_back || target_pokemon.switch_flag.is_some()
    };

    if should_always_hit {
        // move.accuracy = true;
        if let Some(ref mut active_move) = battle.active_move {
            active_move.accuracy = crate::dex::Accuracy::AlwaysHits;
        }
    }

    EventResult::Continue
}

/// onTryHit(target, pokemon) {
///     target.side.removeSideCondition('pursuit');
/// }
pub fn on_try_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),
    _source_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;

    let target = target_pos;

    // target.side.removeSideCondition('pursuit');
    let target_side = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.side_index
    };

    battle.sides[target_side].remove_side_condition(&ID::from("pursuit"));

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onBeforeSwitchOut(pokemon) {
    ///     this.debug('Pursuit start');
    ///     let alreadyAdded = false;
    ///     pokemon.removeVolatile('destinybond');
    ///     for (const source of this.effectState.sources) {
    ///         if (!source.isAdjacent(pokemon) || !this.queue.cancelMove(source) || !source.hp) continue;
    ///         if (!alreadyAdded) {
    ///             this.add('-activate', pokemon, 'move: Pursuit');
    ///             alreadyAdded = true;
    ///         }
    ///         // Run through each action in queue to check if the Pursuit user is supposed to Mega Evolve this turn.
    ///         // If it is, then Mega Evolve before moving.
    ///         if (source.canMegaEvo || source.canUltraBurst || source.canTerastallize) {
    ///             for (const [actionIndex, action] of this.queue.entries()) {
    ///                 if (action.pokemon === source) {
    ///                     if (action.choice === 'megaEvo') {
    ///                         this.actions.runMegaEvo(source);
    ///                     } else if (action.choice === 'terastallize') {
    ///                         // Also a "forme" change that happens before moves, though only possible in NatDex
    ///                         this.actions.terastallize(source);
    ///                     } else {
    ///                         continue;
    ///                     }
    ///                     this.queue.list.splice(actionIndex, 1);
    ///                     break;
    ///                 }
    ///             }
    ///         }
    ///         this.actions.runMove('pursuit', source, source.getLocOf(pokemon));
    ///     }
    /// }
    pub fn on_before_switch_out(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        use crate::battle_actions::{run_mega_evo, terastallize, run_move};
        use crate::battle_queue::Action;

        // this.debug('Pursuit start');
        battle.debug("Pursuit start");

        // pokemon.removeVolatile('destinybond');
        let destinybond_id = ID::from("destinybond");
        Pokemon::remove_volatile(battle, pokemon_pos, &destinybond_id);

        // Get the sources array from the pursuit side condition
        // for (const source of this.effectState.sources)
        let sources = {
            let pursuit_id = ID::from("pursuit");
            let side_idx = pokemon_pos.0;

            if let Some(state) = battle.get_side_condition_data_mut(side_idx, &pursuit_id) {
                // Get sources - clone to avoid borrow issues
                state.sources.clone().unwrap_or_default()
            } else {
                Vec::new()
            }
        };

        let mut already_added = false;

        for source_pos in sources {
            // if (!source.isAdjacent(pokemon) || !this.queue.cancelMove(source) || !source.hp) continue;

            // Check if source is adjacent to pokemon
            let is_adjacent = battle.is_adjacent(source_pos, pokemon_pos);
            if !is_adjacent {
                continue;
            }

            // Check if we can cancel the source's move
            let cancelled = battle.queue.cancel_move(source_pos.0, source_pos.1);
            if !cancelled {
                continue;
            }

            // Check if source has HP
            let has_hp = {
                if let Some(source_pokemon) = battle.pokemon_at(source_pos.0, source_pos.1) {
                    source_pokemon.hp > 0
                } else {
                    false
                }
            };
            if !has_hp {
                continue;
            }

            // if (!alreadyAdded) {
            //     this.add('-activate', pokemon, 'move: Pursuit');
            //     alreadyAdded = true;
            // }
            if !already_added {
                if let Some(target_pokemon) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                    let target_ident = format!("p{}a: {}", pokemon_pos.0 + 1, target_pokemon.set.species);
                    battle.add("-activate", &[
                        crate::battle::Arg::String(target_ident),
                        crate::battle::Arg::Str("move: Pursuit"),
                    ]);
                }
                already_added = true;
            }

            // Check if source can mega evolve / ultra burst / terastallize
            // if (source.canMegaEvo || source.canUltraBurst || source.canTerastallize) {
            let (can_mega, can_tera) = {
                if let Some(source_pokemon) = battle.pokemon_at(source_pos.0, source_pos.1) {
                    (source_pokemon.can_mega_evo.is_some(), source_pokemon.can_terastallize.is_some())
                } else {
                    (false, false)
                }
            };

            if can_mega || can_tera {
                // for (const [actionIndex, action] of this.queue.entries()) {
                //     if (action.pokemon === source) {
                //         if (action.choice === 'megaEvo') {
                //             this.actions.runMegaEvo(source);
                //         } else if (action.choice === 'terastallize') {
                //             this.actions.terastallize(source);
                //         } else {
                //             continue;
                //         }
                //         this.queue.list.splice(actionIndex, 1);
                //         break;
                //     }
                // }

                // Find and remove mega/tera actions for this source
                let mut action_to_remove: Option<usize> = None;
                let mut is_mega = false;
                let mut is_tera = false;

                for (idx, action) in battle.queue.list.iter().enumerate() {
                    // Check if this action is for the source Pokemon
                    let matches = match action {
                        Action::Move(ma) => {
                            (ma.side_index, ma.pokemon_index) == source_pos
                        }
                        Action::Pokemon(pa) => {
                            if (pa.side_index, pa.pokemon_index) == source_pos {
                                match pa.choice {
                                    crate::battle_queue::PokemonActionType::MegaEvo |
                                    crate::battle_queue::PokemonActionType::MegaEvoX |
                                    crate::battle_queue::PokemonActionType::MegaEvoY => {
                                        is_mega = true;
                                        true
                                    }
                                    crate::battle_queue::PokemonActionType::Terastallize => {
                                        is_tera = true;
                                        true
                                    }
                                    _ => false,
                                }
                            } else {
                                false
                            }
                        }
                        _ => false,
                    };

                    if matches && (is_mega || is_tera) {
                        action_to_remove = Some(idx);
                        break;
                    }
                }

                // Run the mega/tera action
                if is_mega {
                    run_mega_evo(battle, source_pos.0, source_pos.1);
                } else if is_tera {
                    terastallize(battle, source_pos);
                }

                // Remove the action from queue
                if let Some(idx) = action_to_remove {
                    battle.queue.list.remove(idx);
                }
            }

            // this.actions.runMove('pursuit', source, source.getLocOf(pokemon));
            let target_loc = {
                if let Some(source_pokemon) = battle.pokemon_at(source_pos.0, source_pos.1) {
                    let active_per_half = battle.sides[0].active.len();
                    source_pokemon.get_loc_of(pokemon_pos.0, pokemon_pos.1, active_per_half)
                } else {
                    0 // Default location
                }
            };

            if let Some(pursuit_data) = battle.dex.moves().get("pursuit").cloned() {
                run_move(battle, &pursuit_data, source_pos, target_loc, None, None, false, None, None, false);
            }
        }

        EventResult::Continue
    }
}

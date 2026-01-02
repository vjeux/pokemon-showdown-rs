//! Rapid Spin Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onAfterHit(target, pokemon, move) {
///     if (!move.hasSheerForce) {
///         if (pokemon.hp && pokemon.removeVolatile('leechseed')) {
///             this.add('-end', pokemon, 'Leech Seed', '[from] move: Rapid Spin', `[of] ${pokemon}`);
///         }
///         const sideConditions = ['spikes', 'toxicspikes', 'stealthrock', 'stickyweb', 'gmaxsteelsurge'];
///         for (const condition of sideConditions) {
///             if (pokemon.hp && pokemon.side.removeSideCondition(condition)) {
///                 this.add('-sideend', pokemon.side, this.dex.conditions.get(condition).name, '[from] move: Rapid Spin', `[of] ${pokemon}`);
///             }
///         }
///         if (pokemon.hp && pokemon.volatiles['partiallytrapped']) {
///             pokemon.removeVolatile('partiallytrapped');
///         }
///     }
/// }
pub fn on_after_hit(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = source_pos;

    // if (!move.hasSheerForce) {
    let has_sheer_force = match &battle.active_move {
        Some(active_move) => active_move.has_sheer_force,
        None => return EventResult::Continue,
    };

    if has_sheer_force {
        return EventResult::Continue;
    }

    // if (pokemon.hp && pokemon.removeVolatile('leechseed')) {
    let (pokemon_hp, pokemon_slot, pokemon_side) = {
        let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon_ref.hp, pokemon_ref.get_slot(), pokemon_ref.side_index)
    };

    if pokemon_hp > 0 {
        let removed_leechseed = {
            let pokemon_mut = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            if pokemon_mut.has_volatile(&ID::from("leechseed")) {
                Pokemon::remove_volatile(battle, (pokemon_mut.side_index, pokemon_mut.position), &ID::from("leechseed"));
                true
            } else {
                false
            }
        };

        if removed_leechseed {
            // this.add('-end', pokemon, 'Leech Seed', '[from] move: Rapid Spin', `[of] ${pokemon}`);
            battle.add(
                "-end",
                &[
                    crate::battle::Arg::from(pokemon_slot.clone()),
                    crate::battle::Arg::from("Leech Seed"),
                    crate::battle::Arg::from("[from] move: Rapid Spin"),
                    crate::battle::Arg::from(format!("[of] {}", pokemon_slot)),
                ],
            );
        }
    }

    // const sideConditions = ['spikes', 'toxicspikes', 'stealthrock', 'stickyweb', 'gmaxsteelsurge'];
    let side_conditions = [
        "spikes",
        "toxicspikes",
        "stealthrock",
        "stickyweb",
        "gmaxsteelsurge",
    ];

    // for (const condition of sideConditions) {
    for condition_id in side_conditions.iter() {
        // if (pokemon.hp && pokemon.side.removeSideCondition(condition)) {
        let pokemon_hp = {
            let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_ref.hp
        };

        if pokemon_hp > 0 {
            let removed = battle.sides[pokemon_side].remove_side_condition(&ID::from(*condition_id));

            if removed {
                // this.add('-sideend', pokemon.side, this.dex.conditions.get(condition).name, '[from] move: Rapid Spin', `[of] ${pokemon}`);
                let condition_name = condition_id.to_string();

                let side_id = if pokemon_side == 0 { "p1" } else { "p2" };
                let pokemon_slot = {
                    let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    pokemon_ref.get_slot()
                };

                battle.add(
                    "-sideend",
                    &[
                        crate::battle::Arg::Str(side_id),
                        crate::battle::Arg::from(condition_name),
                        crate::battle::Arg::from("[from] move: Rapid Spin"),
                        crate::battle::Arg::from(format!("[of] {}", pokemon_slot)),
                    ],
                );
            }
        }
    }

    // if (pokemon.hp && pokemon.volatiles['partiallytrapped']) {
    //     pokemon.removeVolatile('partiallytrapped');
    // }
    let pokemon_hp = {
        let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_ref.hp
    };

    if pokemon_hp > 0 {
        let pokemon_mut = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        if pokemon_mut.has_volatile(&ID::from("partiallytrapped")) {
            Pokemon::remove_volatile(battle, (pokemon_mut.side_index, pokemon_mut.position), &ID::from("partiallytrapped"));
        }
    }

    EventResult::Continue
}

/// onAfterSubDamage(damage, target, pokemon, move) {
///     if (!move.hasSheerForce) {
///         if (pokemon.hp && pokemon.removeVolatile('leechseed')) {
///             this.add('-end', pokemon, 'Leech Seed', '[from] move: Rapid Spin', `[of] ${pokemon}`);
///         }
///         const sideConditions = ['spikes', 'toxicspikes', 'stealthrock', 'stickyweb', 'gmaxsteelsurge'];
///         for (const condition of sideConditions) {
///             if (pokemon.hp && pokemon.side.removeSideCondition(condition)) {
///                 this.add('-sideend', pokemon.side, this.dex.conditions.get(condition).name, '[from] move: Rapid Spin', `[of] ${pokemon}`);
///             }
///         }
///         if (pokemon.hp && pokemon.volatiles['partiallytrapped']) {
///             pokemon.removeVolatile('partiallytrapped');
///         }
///     }
/// }
pub fn on_after_sub_damage(
    battle: &mut Battle,
    _damage: i32,
    _target_pos: Option<(usize, usize)>,
    pokemon_pos: (usize, usize),
    _move_id: &str,
) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = pokemon_pos;

    // if (!move.hasSheerForce) {
    let has_sheer_force = match &battle.active_move {
        Some(active_move) => active_move.has_sheer_force,
        None => return EventResult::Continue,
    };

    if has_sheer_force {
        return EventResult::Continue;
    }

    // if (pokemon.hp && pokemon.removeVolatile('leechseed')) {
    let (pokemon_hp, pokemon_slot, pokemon_side) = {
        let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon_ref.hp, pokemon_ref.get_slot(), pokemon_ref.side_index)
    };

    if pokemon_hp > 0 {
        let removed_leechseed = {
            let pokemon_mut = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            if pokemon_mut.has_volatile(&ID::from("leechseed")) {
                Pokemon::remove_volatile(battle, (pokemon_mut.side_index, pokemon_mut.position), &ID::from("leechseed"));
                true
            } else {
                false
            }
        };

        if removed_leechseed {
            // this.add('-end', pokemon, 'Leech Seed', '[from] move: Rapid Spin', `[of] ${pokemon}`);
            battle.add(
                "-end",
                &[
                    crate::battle::Arg::from(pokemon_slot.clone()),
                    crate::battle::Arg::from("Leech Seed"),
                    crate::battle::Arg::from("[from] move: Rapid Spin"),
                    crate::battle::Arg::from(format!("[of] {}", pokemon_slot)),
                ],
            );
        }
    }

    // const sideConditions = ['spikes', 'toxicspikes', 'stealthrock', 'stickyweb', 'gmaxsteelsurge'];
    let side_conditions = [
        "spikes",
        "toxicspikes",
        "stealthrock",
        "stickyweb",
        "gmaxsteelsurge",
    ];

    // for (const condition of sideConditions) {
    for condition_id in side_conditions.iter() {
        // if (pokemon.hp && pokemon.side.removeSideCondition(condition)) {
        let pokemon_hp = {
            let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_ref.hp
        };

        if pokemon_hp > 0 {
            let removed = battle.sides[pokemon_side].remove_side_condition(&ID::from(*condition_id));

            if removed {
                // this.add('-sideend', pokemon.side, this.dex.conditions.get(condition).name, '[from] move: Rapid Spin', `[of] ${pokemon}`);
                let condition_name = condition_id.to_string();

                let side_id = if pokemon_side == 0 { "p1" } else { "p2" };
                let pokemon_slot = {
                    let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    pokemon_ref.get_slot()
                };

                battle.add(
                    "-sideend",
                    &[
                        crate::battle::Arg::Str(side_id),
                        crate::battle::Arg::from(condition_name),
                        crate::battle::Arg::from("[from] move: Rapid Spin"),
                        crate::battle::Arg::from(format!("[of] {}", pokemon_slot)),
                    ],
                );
            }
        }
    }

    // if (pokemon.hp && pokemon.volatiles['partiallytrapped']) {
    //     pokemon.removeVolatile('partiallytrapped');
    // }
    let pokemon_hp = {
        let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_ref.hp
    };

    if pokemon_hp > 0 {
        let pokemon_mut = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        if pokemon_mut.has_volatile(&ID::from("partiallytrapped")) {
            Pokemon::remove_volatile(battle, (pokemon_mut.side_index, pokemon_mut.position), &ID::from("partiallytrapped"));
        }
    }

    EventResult::Continue
}

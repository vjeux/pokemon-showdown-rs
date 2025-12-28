//! Mortal Spin Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAfterHit(target, pokemon, move) {
///     if (!move.hasSheerForce) {
///         if (pokemon.hp && pokemon.removeVolatile('leechseed')) {
///             this.add('-end', pokemon, 'Leech Seed', '[from] move: Mortal Spin', `[of] ${pokemon}`);
///         }
///         const sideConditions = ['spikes', 'toxicspikes', 'stealthrock', 'stickyweb', 'gmaxsteelsurge'];
///         for (const condition of sideConditions) {
///             if (pokemon.hp && pokemon.side.removeSideCondition(condition)) {
///                 this.add('-sideend', pokemon.side, this.dex.conditions.get(condition).name, '[from] move: Mortal Spin', `[of] ${pokemon}`);
///             }
///         }
///         if (pokemon.hp && pokemon.volatiles['partiallytrapped']) {
///             pokemon.removeVolatile('partiallytrapped');
///         }
///     }
/// }
pub fn on_after_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = source_pos;

    // if (!move.hasSheerForce) {
    let has_sheer_force = {
        let active_move = match &battle.active_move {
            Some(active_move) => active_move,
            None => return EventResult::Continue,
        };
        active_move.has_sheer_force
    };

    if has_sheer_force {
        return EventResult::Continue;
    }

    // if (pokemon.hp && pokemon.removeVolatile('leechseed')) {
    let pokemon_hp = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.hp
    };

    if pokemon_hp > 0 {
        let removed_leechseed = {
            let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.remove_volatile(&ID::from("leechseed"))
        };

        if removed_leechseed {
            // this.add('-end', pokemon, 'Leech Seed', '[from] move: Mortal Spin', `[of] ${pokemon}`);
            let pokemon_arg = {
                let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                crate::battle::Arg::from(pokemon_pokemon)
            };

            battle.add("-end", &[
                pokemon_arg,
                "Leech Seed".into(),
                "[from] move: Mortal Spin".into(),
                format!("[of] {}", pokemon_arg).into(),
            ]);
        }
    }

    // const sideConditions = ['spikes', 'toxicspikes', 'stealthrock', 'stickyweb', 'gmaxsteelsurge'];
    let side_conditions = [
        ID::from("spikes"),
        ID::from("toxicspikes"),
        ID::from("stealthrock"),
        ID::from("stickyweb"),
        ID::from("gmaxsteelsurge"),
    ];

    // for (const condition of sideConditions) {
    for condition in &side_conditions {
        let pokemon_hp = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.hp
        };

        if pokemon_hp > 0 {
            // if (pokemon.hp && pokemon.side.removeSideCondition(condition)) {
            let removed = battle.remove_side_condition(pokemon.0, condition);

            if removed {
                // this.add('-sideend', pokemon.side, this.dex.conditions.get(condition).name, '[from] move: Mortal Spin', `[of] ${pokemon}`);
                let condition_name = battle.dex.get_condition_by_id(condition)
                    .map(|c| c.name.clone())
                    .unwrap_or_else(|| condition.to_string());

                let (side_arg, pokemon_arg) = {
                    let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    let side_id = if pokemon.0 == 0 { "p1" } else { "p2" };
                    let side_arg = crate::battle::Arg::Str(side_id);
                    let pokemon_arg = crate::battle::Arg::from(pokemon_pokemon);
                    (side_arg, pokemon_arg)
                };

                battle.add("-sideend", &[
                    side_arg,
                    condition_name.into(),
                    "[from] move: Mortal Spin".into(),
                    format!("[of] {}", pokemon_arg).into(),
                ]);
            }
        }
    }

    // if (pokemon.hp && pokemon.volatiles['partiallytrapped']) {
    let pokemon_hp = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.hp
    };

    if pokemon_hp > 0 {
        let has_partiallytrapped = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.volatiles.contains_key(&ID::from("partiallytrapped"))
        };

        if has_partiallytrapped {
            // pokemon.removeVolatile('partiallytrapped');
            let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.remove_volatile(&ID::from("partiallytrapped"));
        }
    }

    EventResult::Continue
}

/// onAfterSubDamage(damage, target, pokemon, move) {
///     if (!move.hasSheerForce) {
///         if (pokemon.hp && pokemon.removeVolatile('leechseed')) {
///             this.add('-end', pokemon, 'Leech Seed', '[from] move: Mortal Spin', `[of] ${pokemon}`);
///         }
///         const sideConditions = ['spikes', 'toxicspikes', 'stealthrock', 'stickyweb', 'gmaxsteelsurge'];
///         for (const condition of sideConditions) {
///             if (pokemon.hp && pokemon.side.removeSideCondition(condition)) {
///                 this.add('-sideend', pokemon.side, this.dex.conditions.get(condition).name, '[from] move: Mortal Spin', `[of] ${pokemon}`);
///             }
///         }
///         if (pokemon.hp && pokemon.volatiles['partiallytrapped']) {
///             pokemon.removeVolatile('partiallytrapped');
///         }
///     }
/// }
pub fn on_after_sub_damage(battle: &mut Battle, damage: i32, target_pos: Option<(usize, usize)>, pokemon_pos: (usize, usize), move_id: &str) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = pokemon_pos;

    // if (!move.hasSheerForce) {
    let has_sheer_force = {
        let active_move = match &battle.active_move {
            Some(active_move) => active_move,
            None => return EventResult::Continue,
        };
        active_move.has_sheer_force
    };

    if has_sheer_force {
        return EventResult::Continue;
    }

    // if (pokemon.hp && pokemon.removeVolatile('leechseed')) {
    let pokemon_hp = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.hp
    };

    if pokemon_hp > 0 {
        let removed_leechseed = {
            let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.remove_volatile(&ID::from("leechseed"))
        };

        if removed_leechseed {
            // this.add('-end', pokemon, 'Leech Seed', '[from] move: Mortal Spin', `[of] ${pokemon}`);
            let pokemon_arg = {
                let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                crate::battle::Arg::from(pokemon_pokemon)
            };

            battle.add("-end", &[
                pokemon_arg,
                "Leech Seed".into(),
                "[from] move: Mortal Spin".into(),
                format!("[of] {}", pokemon_arg).into(),
            ]);
        }
    }

    // const sideConditions = ['spikes', 'toxicspikes', 'stealthrock', 'stickyweb', 'gmaxsteelsurge'];
    let side_conditions = [
        ID::from("spikes"),
        ID::from("toxicspikes"),
        ID::from("stealthrock"),
        ID::from("stickyweb"),
        ID::from("gmaxsteelsurge"),
    ];

    // for (const condition of sideConditions) {
    for condition in &side_conditions {
        let pokemon_hp = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.hp
        };

        if pokemon_hp > 0 {
            // if (pokemon.hp && pokemon.side.removeSideCondition(condition)) {
            let removed = battle.remove_side_condition(pokemon.0, condition);

            if removed {
                // this.add('-sideend', pokemon.side, this.dex.conditions.get(condition).name, '[from] move: Mortal Spin', `[of] ${pokemon}`);
                let condition_name = battle.dex.get_condition_by_id(condition)
                    .map(|c| c.name.clone())
                    .unwrap_or_else(|| condition.to_string());

                let (side_arg, pokemon_arg) = {
                    let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    let side_id = if pokemon.0 == 0 { "p1" } else { "p2" };
                    let side_arg = crate::battle::Arg::Str(side_id);
                    let pokemon_arg = crate::battle::Arg::from(pokemon_pokemon);
                    (side_arg, pokemon_arg)
                };

                battle.add("-sideend", &[
                    side_arg,
                    condition_name.into(),
                    "[from] move: Mortal Spin".into(),
                    format!("[of] {}", pokemon_arg).into(),
                ]);
            }
        }
    }

    // if (pokemon.hp && pokemon.volatiles['partiallytrapped']) {
    let pokemon_hp = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.hp
    };

    if pokemon_hp > 0 {
        let has_partiallytrapped = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.volatiles.contains_key(&ID::from("partiallytrapped"))
        };

        if has_partiallytrapped {
            // pokemon.removeVolatile('partiallytrapped');
            let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.remove_volatile(&ID::from("partiallytrapped"));
        }
    }

    EventResult::Continue
}


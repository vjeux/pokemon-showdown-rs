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
pub fn on_after_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    let pokemon_pos = source_pos;

    // if (!move.hasSheerForce) {
    let has_sheer_force = battle.active_move.as_ref().map(|m| m.has_sheer_force).unwrap_or(false);

    if !has_sheer_force {
        // if (pokemon.hp && pokemon.removeVolatile('leechseed')) {
        //     this.add('-end', pokemon, 'Leech Seed', '[from] move: Mortal Spin', `[of] ${pokemon}`);
        // }
        let has_hp = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.hp > 0
        };

        if has_hp && battle.remove_volatile(pokemon_pos, &ID::from("leechseed")) {
            let pokemon_arg = {
                let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                crate::battle::Arg::from(pokemon)
            };
            battle.add("-end", &[
                pokemon_arg.clone(),
                crate::battle::Arg::Effect(ID::from("leechseed")),
                crate::battle::Arg::from("[from] move: Mortal Spin"),
                crate::battle::Arg::from(format!("[of] {}", pokemon_arg.as_string())),
            ]);
        }

        // const sideConditions = ['spikes', 'toxicspikes', 'stealthrock', 'stickyweb', 'gmaxsteelsurge'];
        // for (const condition of sideConditions) {
        //     if (pokemon.hp && pokemon.side.removeSideCondition(condition)) {
        //         this.add('-sideend', pokemon.side, this.dex.conditions.get(condition).name, '[from] move: Mortal Spin', `[of] ${pokemon}`);
        //     }
        // }
        let side_conditions = vec!["spikes", "toxicspikes", "stealthrock", "stickyweb", "gmaxsteelsurge"];

        for condition in side_conditions {
            let has_hp = {
                let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                pokemon.hp > 0
            };

            if has_hp && battle.remove_side_condition(pokemon_pos.0, &ID::from(condition)) {
                let pokemon_arg = {
                    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    crate::battle::Arg::from(pokemon)
                };
                let side_arg = {
                    let side = &battle.sides[pokemon_pos.0];
                    crate::battle::Arg::Side(side.id.clone())
                };
                battle.add("-sideend", &[
                    side_arg,
                    crate::battle::Arg::from(condition),
                    crate::battle::Arg::from("[from] move: Mortal Spin"),
                    crate::battle::Arg::from(format!("[of] {}", pokemon_arg.as_string())),
                ]);
            }
        }

        // if (pokemon.hp && pokemon.volatiles['partiallytrapped']) {
        //     pokemon.removeVolatile('partiallytrapped');
        // }
        let has_hp = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.hp > 0
        };

        if has_hp {
            battle.remove_volatile(pokemon_pos, &ID::from("partiallytrapped"));
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
pub fn on_after_sub_damage(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize)) -> EventResult {
    on_after_hit(battle, target_pos, source_pos)
}


//! Mortal Spin Move
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
pub fn on_after_hit(
    battle: &mut Battle,
    _target_pos: (usize, usize),
    source_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;

    let pokemon_pos = source_pos;

    // if (!move.hasSheerForce) {
    let has_sheer_force = battle
        .active_move
        .as_ref()
        .map(|m| m.borrow().has_sheer_force)
        .unwrap_or(false);

    if !has_sheer_force {
        // if (pokemon.hp && pokemon.removeVolatile('leechseed')) {
        //     this.add('-end', pokemon, 'Leech Seed', '[from] move: Mortal Spin', `[of] ${pokemon}`);
        // }
        let (has_hp, pokemon_ident) = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let hp = pokemon.hp > 0;
            let ident = pokemon.get_slot();
            (hp, ident)
        };

        let removed_leechseed = if has_hp {
            Pokemon::remove_volatile(battle, pokemon_pos, &ID::from("leechseed"))
        } else {
            false
        };

        if has_hp && removed_leechseed {
            battle.add(
                "-end",
                &[
                    pokemon_ident.clone().into(),
                    "Leech Seed".into(),
                    "[from] move: Mortal Spin".into(),
                    format!("[of] {}", pokemon_ident).into(),
                ],
            );
        }

        // const sideConditions = ['spikes', 'toxicspikes', 'stealthrock', 'stickyweb', 'gmaxsteelsurge'];
        // for (const condition of sideConditions) {
        //     if (pokemon.hp && pokemon.side.removeSideCondition(condition)) {
        //         this.add('-sideend', pokemon.side, this.dex.conditions.get(condition).name, '[from] move: Mortal Spin', `[of] ${pokemon}`);
        //     }
        // }
        let side_conditions = vec![
            "spikes",
            "toxicspikes",
            "stealthrock",
            "stickyweb",
            "gmaxsteelsurge",
        ];
        let side_idx = pokemon_pos.0;

        for condition in side_conditions {
            let has_hp = {
                let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                pokemon.hp > 0
            };

            if !has_hp {
                continue;
            }

            let removed = if side_idx < battle.sides.len() {
                battle.sides[side_idx].remove_side_condition(&ID::from(condition))
            } else {
                false
            };

            if removed {
                let pokemon_ident = {
                    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    pokemon.get_slot()
                };

                let side_id = if side_idx == 0 { "p1" } else { "p2" };
                battle.add(
                    "-sideend",
                    &[
                        side_id.into(),
                        condition.into(),
                        "[from] move: Mortal Spin".into(),
                        format!("[of] {}", pokemon_ident).into(),
                    ],
                );
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
            Pokemon::remove_volatile(battle, pokemon_pos, &ID::from("partiallytrapped"));
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
pub fn on_after_sub_damage(
    battle: &mut Battle,
    _damage: i32,
    target_pos: (usize, usize),
    source_pos: (usize, usize),
) -> EventResult {
    on_after_hit(battle, target_pos, source_pos)
}

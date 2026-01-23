//! Tidy Up Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onHit(pokemon) {
///     let success = false;
///     for (const active of this.getAllActive()) {
///         if (active.removeVolatile('substitute')) success = true;
///     }
///     const removeAll = ['spikes', 'toxicspikes', 'stealthrock', 'stickyweb', 'gmaxsteelsurge'];
///     const sides = [pokemon.side, ...pokemon.side.foeSidesWithConditions()];
///     for (const side of sides) {
///         for (const sideCondition of removeAll) {
///             if (side.removeSideCondition(sideCondition)) {
///                 this.add('-sideend', side, this.dex.conditions.get(sideCondition).name);
///                 success = true;
///             }
///         }
///     }
///     if (success) this.add('-activate', pokemon, 'move: Tidy Up');
///     return !!this.boost({ atk: 1, spe: 1 }, pokemon, pokemon, null, false, true) || success;
/// }
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = pokemon_pos;

    // let success = false;
    let mut success = false;

    // for (const active of this.getAllActive()) {
    //     if (active.removeVolatile('substitute')) success = true;
    // }
    let all_active = battle.get_all_active(false);
    for active_pos in all_active {
        let has_substitute = {
            let active = match battle.pokemon_at(active_pos.0, active_pos.1) {
                Some(p) => p,
                None => continue,
            };
            active.has_volatile(&ID::from("substitute"))
        };

        if has_substitute {
            Pokemon::remove_volatile(battle, active_pos, &ID::from("substitute"));
            success = true;
        }
    }

    // const removeAll = ['spikes', 'toxicspikes', 'stealthrock', 'stickyweb', 'gmaxsteelsurge'];
    let remove_all = [
        "spikes",
        "toxicspikes",
        "stealthrock",
        "stickyweb",
        "gmaxsteelsurge",
    ];

    // const sides = [pokemon.side, ...pokemon.side.foeSidesWithConditions()];
    let pokemon_side = {
        let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_ref.side_index
    };

    let mut sides_to_check = vec![pokemon_side];
    let foe_sides = {
        let pokemon_side_ref = &battle.sides[pokemon_side];
        pokemon_side_ref.foe_sides_with_conditions(&battle.sides)
    };
    for foe_side in foe_sides {
        sides_to_check.push(foe_side.n);
    }

    // for (const side of sides) {
    for side_index in sides_to_check {
        // for (const sideCondition of removeAll) {
        for condition_id in remove_all.iter() {
            // if (side.removeSideCondition(sideCondition)) {
            let removed = battle.sides[side_index].remove_side_condition(&ID::from(*condition_id));

            if removed {
                // this.add('-sideend', side, this.dex.conditions.get(sideCondition).name);
                let side_id = if side_index == 0 { "p1" } else { "p2" };
                let condition_name = condition_id.to_string();

                battle.add(
                    "-sideend",
                    &[
                        crate::battle::Arg::Str(side_id),
                        crate::battle::Arg::from(condition_name),
                    ],
                );
                success = true;
            }
        }
    }

    // if (success) this.add('-activate', pokemon, 'move: Tidy Up');
    if success {
        let pokemon_slot = {
            let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_ref.get_slot()
        };

        battle.add(
            "-activate",
            &[
                crate::battle::Arg::from(pokemon_slot),
                crate::battle::Arg::from("move: Tidy Up"),
            ],
        );
    }

    // return !!this.boost({ atk: 1, spe: 1 }, pokemon, pokemon, null, false, true) || success;
    let boost_success = battle.boost(&[("atk", 1), ("spe", 1)], pokemon, Some(pokemon), None, false, false);

    // In JavaScript, this returns `!!boost_result || success`
    // If boost fails (stats maxed) and no subs/hazards were cleared, returns false
    EventResult::Boolean(boost_success || success)
}

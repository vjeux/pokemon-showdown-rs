//! BattleActions::hitStepStealBoosts - Steal positive stat boosts
//!
//! 1:1 port of hitStepStealBoosts from battle-actions.ts

use crate::*;
use crate::battle_actions::ActiveMove;
use crate::dex_data::BoostsTable;

/// Steal positive stat boosts from the target
/// Equivalent to battle-actions.ts hitStepStealBoosts()
///
/// hitStepStealBoosts(targets: Pokemon[], pokemon: Pokemon, move: ActiveMove) {
///     const target = targets[0]; // hardcoded
///     if (move.stealsBoosts) {
///         const boosts: SparseBoostsTable = {};
///         let stolen = false;
///         let statName: BoostID;
///         for (statName in target.boosts) {
///             const stage = target.boosts[statName];
///             if (stage > 0) {
///                 boosts[statName] = stage;
///                 stolen = true;
///             }
///         }
///         if (stolen) {
///             this.battle.attrLastMove('[still]');
///             this.battle.add('-clearpositiveboost', target, pokemon, 'move: ' + move.name);
///             this.battle.boost(boosts, pokemon, pokemon);
///
///             let statName2: BoostID;
///             for (statName2 in boosts) {
///                 boosts[statName2] = 0;
///             }
///             target.setBoost(boosts);
///             if (move.id === "spectralthief") {
///                 this.battle.addMove('-anim', pokemon, "Spectral Thief", target);
///             }
///         }
///     }
///     return undefined;
/// }
pub fn hit_step_steal_boosts(
    battle: &mut Battle,
    targets: &[(usize, usize)],
    attacker_pos: (usize, usize),
    active_move: &ActiveMove,
) {
    // const target = targets[0]; // hardcoded
    let target_pos = match targets.first() {
        Some(&pos) => pos,
        None => return,
    };

    // if (move.stealsBoosts) {
    if !active_move.steals_boosts {
        return;
    }

    // const boosts: SparseBoostsTable = {};
    // let stolen = false;
    let mut boosts = BoostsTable::default();
    let mut stolen = false;

    // let statName: BoostID;
    // for (statName in target.boosts) {
    //     const stage = target.boosts[statName];
    //     if (stage > 0) {
    //         boosts[statName] = stage;
    //         stolen = true;
    //     }
    // }
    {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return,
        };

        if target.boosts.atk > 0 {
            boosts.atk = target.boosts.atk;
            stolen = true;
        }
        if target.boosts.def > 0 {
            boosts.def = target.boosts.def;
            stolen = true;
        }
        if target.boosts.spa > 0 {
            boosts.spa = target.boosts.spa;
            stolen = true;
        }
        if target.boosts.spd > 0 {
            boosts.spd = target.boosts.spd;
            stolen = true;
        }
        if target.boosts.spe > 0 {
            boosts.spe = target.boosts.spe;
            stolen = true;
        }
        if target.boosts.accuracy > 0 {
            boosts.accuracy = target.boosts.accuracy;
            stolen = true;
        }
        if target.boosts.evasion > 0 {
            boosts.evasion = target.boosts.evasion;
            stolen = true;
        }
    }

    // if (stolen) {
    if stolen {
        //     this.battle.attrLastMove('[still]');
        battle.attr_last_move(&["[still]"]);

        //     this.battle.add('-clearpositiveboost', target, pokemon, 'move: ' + move.name);
        if let (Some(target_pokemon), Some(attacker_pokemon)) = (
            battle.pokemon_at(target_pos.0, target_pos.1),
            battle.pokemon_at(attacker_pos.0, attacker_pos.1),
        ) {
            let target_ident = format!("p{}a: {}", target_pos.0 + 1, target_pokemon.set.species);
            let attacker_ident = format!("p{}a: {}", attacker_pos.0 + 1, attacker_pokemon.set.species);
            let move_name_msg = format!("move: {}", active_move.name);

            battle.add("-clearpositiveboost", &[
                crate::battle::Arg::String(target_ident),
                crate::battle::Arg::String(attacker_ident),
                crate::battle::Arg::String(move_name_msg),
            ]);
        }

        //     this.battle.boost(boosts, pokemon, pokemon);
        // Convert BoostsTable to battle.boost format: &[(&str, i8)]
        let mut boost_array: Vec<(&str, i8)> = Vec::new();
        if boosts.atk != 0 {
            boost_array.push(("atk", boosts.atk));
        }
        if boosts.def != 0 {
            boost_array.push(("def", boosts.def));
        }
        if boosts.spa != 0 {
            boost_array.push(("spa", boosts.spa));
        }
        if boosts.spd != 0 {
            boost_array.push(("spd", boosts.spd));
        }
        if boosts.spe != 0 {
            boost_array.push(("spe", boosts.spe));
        }
        if boosts.accuracy != 0 {
            boost_array.push(("accuracy", boosts.accuracy));
        }
        if boosts.evasion != 0 {
            boost_array.push(("evasion", boosts.evasion));
        }

        battle.boost(&boost_array, attacker_pos, Some(attacker_pos), None, false, false);

        //     let statName2: BoostID;
        //     for (statName2 in boosts) {
        //         boosts[statName2] = 0;
        //     }
        //     target.setBoost(boosts);
        // Convert boosts to HashMap for set_boost
        use std::collections::HashMap;
        use crate::dex_data::BoostID;

        let mut boosts_map = HashMap::new();
        if boosts.atk > 0 {
            boosts_map.insert(BoostID::Atk, 0);
        }
        if boosts.def > 0 {
            boosts_map.insert(BoostID::Def, 0);
        }
        if boosts.spa > 0 {
            boosts_map.insert(BoostID::SpA, 0);
        }
        if boosts.spd > 0 {
            boosts_map.insert(BoostID::SpD, 0);
        }
        if boosts.spe > 0 {
            boosts_map.insert(BoostID::Spe, 0);
        }
        if boosts.accuracy > 0 {
            boosts_map.insert(BoostID::Accuracy, 0);
        }
        if boosts.evasion > 0 {
            boosts_map.insert(BoostID::Evasion, 0);
        }

        if let Some(target_pokemon) = battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            target_pokemon.set_boost(boosts_map);
        }

        //     if (move.id === "spectralthief") {
        //         this.battle.addMove('-anim', pokemon, "Spectral Thief", target);
        //     }
        if active_move.id.as_str() == "spectralthief" {
            let pokemon_str = {
                let pokemon = match battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
                    Some(p) => p,
                    None => return,
                };
                pokemon.get_slot()
            };
            let target_str = {
                let target_pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                    Some(p) => p,
                    None => return,
                };
                target_pokemon.get_slot()
            };
            battle.add_move(&["-anim", &pokemon_str, "Spectral Thief", &target_str]);
        }
    }

    // return undefined;
}

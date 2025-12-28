//! Defog Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source, move) {
///     let success = false;
///     if (!target.volatiles['substitute'] || move.infiltrates) success = !!this.boost({ evasion: -1 });
///     const removeAll = ['spikes', 'toxicspikes', 'stealthrock', 'stickyweb', 'gmaxsteelsurge'];
///     const removeTarget = ['reflect', 'lightscreen', 'auroraveil', 'safeguard', 'mist', ...removeAll];
///     for (const targetCondition of removeTarget) {
///         if (target.side.removeSideCondition(targetCondition)) {
///             if (!removeAll.includes(targetCondition)) continue;
///             this.add('-sideend', target.side, this.dex.conditions.get(targetCondition).name, '[from] move: Defog', `[of] ${source}`);
///             success = true;
///         }
///     }
///     for (const sideCondition of removeAll) {
///         if (source.side.removeSideCondition(sideCondition)) {
///             this.add('-sideend', source.side, this.dex.conditions.get(sideCondition).name, '[from] move: Defog', `[of] ${source}`);
///             success = true;
///         }
///     }
///     this.field.clearTerrain();
///     return success;
/// }
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    // Get source and target
    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // let success = false;
    let mut success = false;

    // if (!target.volatiles['substitute'] || move.infiltrates) success = !!this.boost({ evasion: -1 });
    let has_substitute = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon
            .volatiles
            .contains_key(&ID::from("substitute"))
    };

    // TODO: Check if move infiltrates - requires access to current move data
    // For now, we'll check substitute status
    if !has_substitute {
        // success = !!this.boost({ evasion: -1 });
        let boosts = [("evasion", -1)];
        let boost_result = battle.boost(&boosts, target, Some(pokemon_pos), None);
        success = boost_result;
    }

    // const removeAll = ['spikes', 'toxicspikes', 'stealthrock', 'stickyweb', 'gmaxsteelsurge'];
    let remove_all = vec![
        "spikes",
        "toxicspikes",
        "stealthrock",
        "stickyweb",
        "gmaxsteelsurge",
    ];

    // const removeTarget = ['reflect', 'lightscreen', 'auroraveil', 'safeguard', 'mist', ...removeAll];
    let mut remove_target = vec!["reflect", "lightscreen", "auroraveil", "safeguard", "mist"];
    remove_target.extend_from_slice(&remove_all);

    // for (const targetCondition of removeTarget) {
    //     if (target.side.removeSideCondition(targetCondition)) {
    //         if (!removeAll.includes(targetCondition)) continue;
    //         this.add('-sideend', target.side, this.dex.conditions.get(targetCondition).name, '[from] move: Defog', `[of] ${source}`);
    //         success = true;
    //     }
    // }
    let target_side_index = target.0;
    for target_condition in &remove_target {
        let condition_id = ID::from(*target_condition);
        let removed = if let Some(target_side) = battle.sides.get_mut(target_side_index) {
            target_side.remove_side_condition(&condition_id)
        } else {
            false
        };

        if removed {
            // if (!removeAll.includes(targetCondition)) continue;
            if !remove_all.contains(target_condition) {
                continue;
            }

            // this.add('-sideend', target.side, this.dex.conditions.get(targetCondition).name, '[from] move: Defog', `[of] ${source}`);
            let (target_side_arg, source_ident, condition_name) = {
                let _target_side = match battle.sides.get(target_side_index) {
                    Some(s) => s,
                    None => continue,
                };
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => continue,
                };

                // Get condition name from dex
                // Conditions don't have separate data, just use the ID string
                let condition_name = condition_id.to_string();

                let side_id = if target_side_index == 0 { "p1" } else { "p2" };
                (
                    crate::battle::Arg::Str(side_id),
                    source_pokemon.get_slot(),
                    condition_name,
                )
            };

            battle.add(
                "-sideend",
                &[
                    target_side_arg,
                    condition_name.into(),
                    "[from] move: Defog".into(),
                    format!("[of] {}", source_ident).into(),
                ],
            );

            // success = true;
            success = true;
        }
    }

    // for (const sideCondition of removeAll) {
    //     if (source.side.removeSideCondition(sideCondition)) {
    //         this.add('-sideend', source.side, this.dex.conditions.get(sideCondition).name, '[from] move: Defog', `[of] ${source}`);
    //         success = true;
    //     }
    // }
    let source_side_index = source.0;
    for side_condition in &remove_all {
        let condition_id = ID::from(*side_condition);
        let removed = if let Some(source_side) = battle.sides.get_mut(source_side_index) {
            source_side.remove_side_condition(&condition_id)
        } else {
            false
        };

        if removed {
            // this.add('-sideend', source.side, this.dex.conditions.get(sideCondition).name, '[from] move: Defog', `[of] ${source}`);
            let (source_side_arg, source_ident, condition_name) = {
                let _source_side = match battle.sides.get(source_side_index) {
                    Some(s) => s,
                    None => continue,
                };
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => continue,
                };

                // Get condition name from dex
                // Conditions don't have separate data, just use the ID string
                let condition_name = condition_id.to_string();

                let side_id = if source_side_index == 0 { "p1" } else { "p2" };
                (
                    crate::battle::Arg::Str(side_id),
                    source_pokemon.get_slot(),
                    condition_name,
                )
            };

            battle.add(
                "-sideend",
                &[
                    source_side_arg,
                    condition_name.into(),
                    "[from] move: Defog".into(),
                    format!("[of] {}", source_ident).into(),
                ],
            );

            // success = true;
            success = true;
        }
    }

    // this.field.clearTerrain();
    battle.field.clear_terrain();

    // return success;
    EventResult::Boolean(success)
}

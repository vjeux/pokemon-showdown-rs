//! G-Max Wind Rage Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onHit(source)
///
/// ```text
/// JS Source (data/moves.ts):
/// onHit(source) {
/// 				let success = false;
/// 				const removeAll = ['spikes', 'toxicspikes', 'stealthrock', 'stickyweb', 'gmaxsteelsurge'];
/// 				const removeTarget = ['reflect', 'lightscreen', 'auroraveil', 'safeguard', 'mist', ...removeAll];
/// 				for (const targetCondition of removeTarget) {
/// 					if (source.side.foe.removeSideCondition(targetCondition)) {
/// 						if (!removeAll.includes(targetCondition)) continue;
/// 						this.add('-sideend', source.side.foe, this.dex.conditions.get(targetCondition).name, '[from] move: G-Max Wind Rage', `[of] ${source}`);
/// 						success = true;
/// 					}
/// 				}
/// 				for (const sideCondition of removeAll) {
/// 					if (source.side.removeSideCondition(sideCondition)) {
/// 						this.add('-sideend', source.side, this.dex.conditions.get(sideCondition).name, '[from] move: G-Max Wind Rage', `[of] ${source}`);
/// 						success = true;
/// 					}
/// 				}
/// 				this.field.clearTerrain();
/// 				return success;
/// 			},
///
/// 		}
/// ```
pub fn on_hit(
    battle: &mut Battle,
    _target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
        _source_effect: Option<&crate::battle::Effect>,
) -> EventResult {
    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // let success = false;
    let mut success = false;

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

    // Get source side and foe side indices
    let source_side_index = source.0;
    let foe_side_index = 1 - source_side_index; // 0 -> 1, 1 -> 0

    // for (const targetCondition of removeTarget) {
    //     if (source.side.foe.removeSideCondition(targetCondition)) {
    //         if (!removeAll.includes(targetCondition)) continue;
    //         this.add('-sideend', source.side.foe, this.dex.conditions.get(targetCondition).name, '[from] move: G-Max Wind Rage', `[of] ${source}`);
    //         success = true;
    //     }
    // }
    for target_condition in &remove_target {
        let condition_id = ID::from(*target_condition);
        let removed = if let Some(foe_side) = battle.sides.get_mut(foe_side_index) {
            foe_side.remove_side_condition(&condition_id)
        } else {
            false
        };

        if removed {
            // if (!removeAll.includes(targetCondition)) continue;
            if !remove_all.contains(target_condition) {
                continue;
            }

            // this.add('-sideend', source.side.foe, this.dex.conditions.get(targetCondition).name, '[from] move: G-Max Wind Rage', `[of] ${source}`);
            let (foe_side_arg, source_ident, condition_name) = {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => continue,
                };

                let condition_name = condition_id.to_string();
                let foe_side_id = if foe_side_index == 0 { "p1" } else { "p2" };

                (
                    crate::battle::Arg::Str(foe_side_id),
                    source_pokemon.get_slot(),
                    condition_name,
                )
            };

            battle.add(
                "-sideend",
                &[
                    foe_side_arg,
                    condition_name.into(),
                    "[from] move: G-Max Wind Rage".into(),
                    format!("[of] {}", source_ident).into(),
                ],
            );

            // success = true;
            success = true;
        }
    }

    // for (const sideCondition of removeAll) {
    //     if (source.side.removeSideCondition(sideCondition)) {
    //         this.add('-sideend', source.side, this.dex.conditions.get(sideCondition).name, '[from] move: G-Max Wind Rage', `[of] ${source}`);
    //         success = true;
    //     }
    // }
    for side_condition in &remove_all {
        let condition_id = ID::from(*side_condition);
        let removed = if let Some(source_side) = battle.sides.get_mut(source_side_index) {
            source_side.remove_side_condition(&condition_id)
        } else {
            false
        };

        if removed {
            // this.add('-sideend', source.side, this.dex.conditions.get(sideCondition).name, '[from] move: G-Max Wind Rage', `[of] ${source}`);
            let (source_side_arg, source_ident, condition_name) = {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => continue,
                };

                let condition_name = condition_id.to_string();
                let source_side_id = if source_side_index == 0 { "p1" } else { "p2" };

                (
                    crate::battle::Arg::Str(source_side_id),
                    source_pokemon.get_slot(),
                    condition_name,
                )
            };

            battle.add(
                "-sideend",
                &[
                    source_side_arg,
                    condition_name.into(),
                    "[from] move: G-Max Wind Rage".into(),
                    format!("[of] {}", source_ident).into(),
                ],
            );

            // success = true;
            success = true;
        }
    }

    // this.field.clearTerrain();
    battle.clear_terrain();

    // return success;
    EventResult::Boolean(success)
}


/// Self-targeting callbacks
/// These callbacks target the move user (source), not the move target
pub mod self_callbacks {
    use super::*;

    /// self.onHit(source)
    ///
    /// ```text
    /// JS Source (data/moves.ts):
    /// self: {
    ///     onHit(source) {
    ///         onHit(source) {
    ///                 let success = false;
    ///                 const removeAll = ["spikes", "toxicspikes", "stealthrock", "stickyweb", "gmaxsteelsurge"];
    ///                 const removeTarget = ["reflect", "lightscreen", "auroraveil", "safeguard", "mist", ...removeAll];
    ///                 for (const targetCondition of removeTarget) {
    ///                   if (source.side.foe.removeSideCondition(targetCondition)) {
    ///                     if (!removeAll.includes(targetCondition)) continue;
    ///                     this.add("-sideend", source.side.foe, this.dex.conditions.get(targetCondition).name, "[from] move: G-Max Wind Rage", `[of] ${source}`);
    ///                     success = true;
    ///                   }
    ///                 }
    ///                 for (const sideCondition of removeAll) {
    ///                   if (source.side.removeSideCondition(sideCondition)) {
    ///                     this.add("-sideend", source.side, this.dex.conditions.get(sideCondition).name, "[from] move: G-Max Wind Rage", `[of] ${source}`);
    ///                     success = true;
    ///                   }
    ///                 }
    ///                 this.field.clearTerrain();
    ///                 return success;
    ///               }
    ///     },
    /// }
    /// ```
    ///
    /// NOTE: For self callbacks, the FIRST parameter receives the move USER (source),
    /// and the SECOND parameter receives the move TARGET (or None).
    /// The naming convention in dispatch_self_on_hit is misleading - it names them
    /// target_pos and source_pos, but actually passes source as first, target as second.
    pub fn on_hit(
        battle: &mut Battle,
        source_pos: (usize, usize),          // ACTUAL SOURCE (move user)
        _target_pos: Option<(usize, usize)>, // ACTUAL TARGET (move target)
        _source_effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
        use crate::dex_data::ID;

        let mut success = false;

        // const removeAll = ["spikes", "toxicspikes", "stealthrock", "stickyweb", "gmaxsteelsurge"];
        let remove_all = vec!["spikes", "toxicspikes", "stealthrock", "stickyweb", "gmaxsteelsurge"];

        // const removeTarget = ["reflect", "lightscreen", "auroraveil", "safeguard", "mist", ...removeAll];
        let mut remove_target = vec!["reflect", "lightscreen", "auroraveil", "safeguard", "mist"];
        remove_target.extend(remove_all.iter());

        let source_side_idx = source_pos.0;
        let foe_side_idx = if source_side_idx == 0 { 1 } else { 0 };

        // for (const targetCondition of removeTarget) {
        //     if (source.side.foe.removeSideCondition(targetCondition)) {
        //         if (!removeAll.includes(targetCondition)) continue;
        //         this.add("-sideend", source.side.foe, this.dex.conditions.get(targetCondition).name, "[from] move: G-Max Wind Rage", `[of] ${source}`);
        //         success = true;
        //     }
        // }
        for target_condition in &remove_target {
            let condition_id = ID::new(target_condition);
            if battle.sides[foe_side_idx].remove_side_condition(&condition_id) {
                if !remove_all.contains(target_condition) {
                    continue;
                }

                // this.add("-sideend", source.side.foe, this.dex.conditions.get(targetCondition).name, "[from] move: G-Max Wind Rage", `[of] ${source}`);
                let (foe_side_arg, source_ident, condition_name) = {
                    let source_pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };

                    let condition_name = condition_id.to_string();
                    let foe_side_id = if foe_side_idx == 0 { "p1" } else { "p2" };

                    (
                        crate::battle::Arg::Str(foe_side_id),
                        source_pokemon.get_slot(),
                        condition_name,
                    )
                };

                battle.add(
                    "-sideend",
                    &[
                        foe_side_arg,
                        condition_name.into(),
                        "[from] move: G-Max Wind Rage".into(),
                        format!("[of] {}", source_ident).into(),
                    ],
                );
                success = true;
            }
        }

        // for (const sideCondition of removeAll) {
        //     if (source.side.removeSideCondition(sideCondition)) {
        //         this.add("-sideend", source.side, this.dex.conditions.get(sideCondition).name, "[from] move: G-Max Wind Rage", `[of] ${source}`);
        //         success = true;
        //     }
        // }
        for side_condition in &remove_all {
            let condition_id = ID::new(side_condition);
            if battle.sides[source_side_idx].remove_side_condition(&condition_id) {
                // this.add("-sideend", source.side, this.dex.conditions.get(sideCondition).name, "[from] move: G-Max Wind Rage", `[of] ${source}`);
                let (source_side_arg, source_ident, condition_name) = {
                    let source_pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };

                    let condition_name = condition_id.to_string();
                    let source_side_id = if source_side_idx == 0 { "p1" } else { "p2" };

                    (
                        crate::battle::Arg::Str(source_side_id),
                        source_pokemon.get_slot(),
                        condition_name,
                    )
                };

                battle.add(
                    "-sideend",
                    &[
                        source_side_arg,
                        condition_name.into(),
                        "[from] move: G-Max Wind Rage".into(),
                        format!("[of] {}", source_ident).into(),
                    ],
                );
                success = true;
            }
        }

        // this.field.clearTerrain();
        battle.clear_terrain();

        // return success;
        if success {
            EventResult::Boolean(true)
        } else {
            EventResult::Continue
        }
    }
}

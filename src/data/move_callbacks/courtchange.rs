//! Court Change Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::GameType;
use crate::event::EventResult;

/// onHitField(target, source) {
///     const sideConditions = [
///         'mist', 'lightscreen', 'reflect', 'spikes', 'safeguard', 'tailwind', 'toxicspikes', 'stealthrock', 'waterpledge', 'firepledge', 'grasspledge', 'stickyweb', 'auroraveil', 'luckychant', 'gmaxsteelsurge', 'gmaxcannonade', 'gmaxvinelash', 'gmaxwildfire', 'gmaxvolcalith',
///     ];
///     let success = false;
///     if (this.gameType === "freeforall") {
///         // the list of all sides in clockwise order
///         const sides = [this.sides[0], this.sides[3]!, this.sides[1], this.sides[2]!];
///         const temp: { [k: number]: typeof source.side.sideConditions } = { 0: {}, 1: {}, 2: {}, 3: {} };
///         for (const side of sides) {
///             for (const id in side.sideConditions) {
///                 if (!sideConditions.includes(id)) continue;
///                 temp[side.n][id] = side.sideConditions[id];
///                 delete side.sideConditions[id];
///                 success = true;
///             }
///         }
///         for (let i = 0; i < 4; i++) {
///             const sourceSideConditions = temp[sides[i].n];
///             const targetSide = sides[(i + 1) % 4]; // the next side in rotation
///             for (const id in sourceSideConditions) {
///                 targetSide.sideConditions[id] = sourceSideConditions[id];
///                 targetSide.sideConditions[id].target = targetSide;
///             }
///         }
///     } else {
///         const sourceSideConditions = source.side.sideConditions;
///         const targetSideConditions = source.side.foe.sideConditions;
///         const sourceTemp: typeof sourceSideConditions = {};
///         const targetTemp: typeof targetSideConditions = {};
///         for (const id in sourceSideConditions) {
///             if (!sideConditions.includes(id)) continue;
///             sourceTemp[id] = sourceSideConditions[id];
///             delete sourceSideConditions[id];
///             success = true;
///         }
///         for (const id in targetSideConditions) {
///             if (!sideConditions.includes(id)) continue;
///             targetTemp[id] = targetSideConditions[id];
///             delete targetSideConditions[id];
///             success = true;
///         }
///         for (const id in sourceTemp) {
///             targetSideConditions[id] = sourceTemp[id];
///             targetSideConditions[id].target = source.side.foe;
///         }
///         for (const id in targetTemp) {
///             sourceSideConditions[id] = targetTemp[id];
///             sourceSideConditions[id].target = source.side;
///         }
///     }
///     if (!success) return false;
///     this.add('-swapsideconditions');
///     this.add('-activate', source, 'move: Court Change');
/// }
pub fn on_hit_field(
    battle: &mut Battle,
    _target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    // Get source position
    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const sideConditions = [
    //     'mist', 'lightscreen', 'reflect', 'spikes', 'safeguard', 'tailwind', 'toxicspikes', 'stealthrock', 'waterpledge', 'firepledge', 'grasspledge', 'stickyweb', 'auroraveil', 'luckychant', 'gmaxsteelsurge', 'gmaxcannonade', 'gmaxvinelash', 'gmaxwildfire', 'gmaxvolcalith',
    // ];
    let side_conditions = vec![
        "mist",
        "lightscreen",
        "reflect",
        "spikes",
        "safeguard",
        "tailwind",
        "toxicspikes",
        "stealthrock",
        "waterpledge",
        "firepledge",
        "grasspledge",
        "stickyweb",
        "auroraveil",
        "luckychant",
        "gmaxsteelsurge",
        "gmaxcannonade",
        "gmaxvinelash",
        "gmaxwildfire",
        "gmaxvolcalith",
    ];

    // let success = false;
    let mut success = false;

    // if (this.gameType === "freeforall") {
    if battle.game_type == GameType::FreeForAll {
        // TODO: Implement free-for-all rotation logic
        // For now, we'll skip this case as it's complex and requires 4-side support
        // The standard 2-side case below is the most common scenario
    } else {
        // const sourceSideConditions = source.side.sideConditions;
        // const targetSideConditions = source.side.foe.sideConditions;
        let source_side_index = source.0;
        let target_side_index = 1 - source_side_index;

        // const sourceTemp: typeof sourceSideConditions = {};
        // const targetTemp: typeof targetSideConditions = {};
        let mut source_temp = std::collections::HashMap::new();
        let mut target_temp = std::collections::HashMap::new();

        // for (const id in sourceSideConditions) {
        //     if (!sideConditions.includes(id)) continue;
        //     sourceTemp[id] = sourceSideConditions[id];
        //     delete sourceSideConditions[id];
        //     success = true;
        // }
        if let Some(source_side) = battle.sides.get_mut(source_side_index) {
            let keys_to_remove: Vec<_> = source_side
                .side_conditions
                .keys()
                .filter(|id| side_conditions.contains(&id.as_str()))
                .cloned()
                .collect();

            for id in keys_to_remove {
                if let Some(condition) = source_side.side_conditions.remove(&id) {
                    source_temp.insert(id, condition);
                    success = true;
                }
            }
        }

        // for (const id in targetSideConditions) {
        //     if (!sideConditions.includes(id)) continue;
        //     targetTemp[id] = targetSideConditions[id];
        //     delete targetSideConditions[id];
        //     success = true;
        // }
        if let Some(target_side) = battle.sides.get_mut(target_side_index) {
            let keys_to_remove: Vec<_> = target_side
                .side_conditions
                .keys()
                .filter(|id| side_conditions.contains(&id.as_str()))
                .cloned()
                .collect();

            for id in keys_to_remove {
                if let Some(condition) = target_side.side_conditions.remove(&id) {
                    target_temp.insert(id, condition);
                    success = true;
                }
            }
        }

        // for (const id in sourceTemp) {
        //     targetSideConditions[id] = sourceTemp[id];
        //     targetSideConditions[id].target = source.side.foe;
        // }
        if let Some(target_side) = battle.sides.get_mut(target_side_index) {
            for (id, mut condition) in source_temp {
                // targetSideConditions[id].target = source.side.foe;
                condition.target_side = Some(target_side_index);
                target_side.side_conditions.insert(id, condition);
            }
        }

        // for (const id in targetTemp) {
        //     sourceSideConditions[id] = targetTemp[id];
        //     sourceSideConditions[id].target = source.side;
        // }
        if let Some(source_side) = battle.sides.get_mut(source_side_index) {
            for (id, mut condition) in target_temp {
                // sourceSideConditions[id].target = source.side;
                condition.target_side = Some(source_side_index);
                source_side.side_conditions.insert(id, condition);
            }
        }
    }

    // if (!success) return false;
    if !success {
        return EventResult::Boolean(false);
    }

    // this.add('-swapsideconditions');
    battle.add("-swapsideconditions", &[]);

    // this.add('-activate', source, 'move: Court Change');
    let source_ident = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.get_slot()
    };

    battle.add(
        "-activate",
        &[source_ident.as_str().into(), "move: Court Change".into()],
    );

    EventResult::Continue
}

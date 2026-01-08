//! Heal Bell Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onHit(target, source) {
///     this.add('-activate', source, 'move: Heal Bell');
///     let success = false;
///     const allies = [...target.side.pokemon, ...target.side.allySide?.pokemon || []];
///     for (const ally of allies) {
///         if (ally !== source && !this.suppressingAbility(ally)) {
///             if (ally.hasAbility('soundproof')) {
///                 this.add('-immune', ally, '[from] ability: Soundproof');
///                 continue;
///             }
///             if (ally.hasAbility('goodasgold')) {
///                 this.add('-immune', ally, '[from] ability: Good as Gold');
///                 continue;
///             }
///         }
///         if (ally.cureStatus()) success = true;
///     }
///     return success;
/// }
pub fn on_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),  // First param is target
    source_pos: Option<(usize, usize)>,  // Second param is source
) -> EventResult {
    let target = target_pos;
    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // this.add('-activate', source, 'move: Heal Bell');
    let source_ident = {
        let pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };
    battle.add(
        "-activate",
        &[source_ident.as_str().into(), "move: Heal Bell".into()],
    );

    // let success = false;
    let mut success = false;

    // const allies = [...target.side.pokemon, ...target.side.allySide?.pokemon || []];
    // Collect allies from both the target's side and the ally side (for multi battles)
    let mut allies: Vec<(usize, usize)> = Vec::new();

    // Add target's side pokemon
    let target_side_active: Vec<(usize, usize)> = battle.sides[target.0]
        .active
        .iter()
        .enumerate()
        .filter_map(|(i, &active)| {
            if active.is_some() {
                Some((target.0, i))
            } else {
                None
            }
        })
        .collect();
    allies.extend(target_side_active);

    // Add ally side pokemon (for multi battles)
    let ally_side_index = battle.sides[target.0].ally_index;
    if let Some(ally_idx) = ally_side_index {
        let ally_side_active: Vec<(usize, usize)> = battle.sides[ally_idx]
            .active
            .iter()
            .enumerate()
            .filter_map(|(i, &active)| {
                if active.is_some() {
                    Some((ally_idx, i))
                } else {
                    None
                }
            })
            .collect();
        allies.extend(ally_side_active);
    }

    // for (const ally of allies) {
    for ally_pos in allies {
        // if (ally !== source && !this.suppressingAbility(ally)) {
        let is_not_source = ally_pos != source;
        let suppressing_ability = if is_not_source {
            battle.suppressing_ability(Some(ally_pos))
        } else {
            false
        };

        if is_not_source && !suppressing_ability {
            // if (ally.hasAbility('soundproof')) {
            let has_soundproof = {
                let ally = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                ally.has_ability(battle, &["soundproof"])
            };

            if has_soundproof {
                // this.add('-immune', ally, '[from] ability: Soundproof');
                let ally_ident = {
                    let pokemon = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
                        Some(p) => p,
                        None => continue,
                    };
                    pokemon.get_slot()
                };
                battle.add(
                    "-immune",
                    &[
                        ally_ident.as_str().into(),
                        "[from] ability: Soundproof".into(),
                    ],
                );
                // continue;
                continue;
            }

            // if (ally.hasAbility('goodasgold')) {
            let has_goodasgold = {
                let ally = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                ally.has_ability(battle, &["goodasgold"])
            };

            if has_goodasgold {
                // this.add('-immune', ally, '[from] ability: Good as Gold');
                let ally_ident = {
                    let pokemon = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
                        Some(p) => p,
                        None => continue,
                    };
                    pokemon.get_slot()
                };
                battle.add(
                    "-immune",
                    &[
                        ally_ident.as_str().into(),
                        "[from] ability: Good as Gold".into(),
                    ],
                );
                // continue;
                continue;
            }
        }

        // if (ally.cureStatus()) success = true;
        let (ally_ident, ally_name) = {
            let ally = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
                Some(p) => p,
                None => continue,
            };
            (ally.get_slot(), ally.name.clone())
        };

        let _ally_mut = match battle.pokemon_at_mut(ally_pos.0, ally_pos.1) {
            Some(p) => p,
            None => continue,
        };

        if let Some((status, removed_nightmare, _silent)) = Pokemon::cure_status(battle, ally_pos, false) {
            let full_name = format!("{}: {}", ally_ident, ally_name);
            battle.add("-curestatus", &[full_name.as_str().into(), status.as_str().into(), "[msg]".into()]);
            if removed_nightmare {
                battle.add("-end", &[full_name.as_str().into(), "Nightmare".into(), "[silent]".into()]);
            }
            success = true;
        }
    }

    // return success;
    EventResult::Boolean(success)
}

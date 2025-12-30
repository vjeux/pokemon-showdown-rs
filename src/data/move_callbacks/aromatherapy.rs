//! Aromatherapy Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onHit(target, source, move) {
///     this.add('-activate', source, 'move: Aromatherapy');
///     let success = false;
///     const allies = [...target.side.pokemon, ...target.side.allySide?.pokemon || []];
///     for (const ally of allies) {
///         if (ally !== source && !this.suppressingAbility(ally)) {
///             if (ally.hasAbility('sapsipper')) {
///                 this.add('-immune', ally, '[from] ability: Sap Sipper');
///                 continue;
///             }
///             if (ally.hasAbility('goodasgold')) {
///                 this.add('-immune', ally, '[from] ability: Good as Gold');
///                 continue;
///             }
///             if (ally.volatiles['substitute'] && !move.infiltrates) continue;
///         }
///         if (ally.cureStatus()) success = true;
///     }
///     return success;
/// }
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    // this.add('-activate', source, 'move: Aromatherapy');
    let source_ident = {
        let source_pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.get_slot()
    };
    battle.add(
        "-activate",
        &[source_ident.as_str().into(), "move: Aromatherapy".into()],
    );

    // let success = false;
    let mut success = false;

    // Get the target position
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const allies = [...target.side.pokemon, ...target.side.allySide?.pokemon || []];
    // For now, just iterate through target.side.pokemon (allySide not yet implemented)
    let target_side_idx = target.0;
    let num_pokemon = battle.sides[target_side_idx].pokemon.len();

    // for (const ally of allies) {
    for ally_idx in 0..num_pokemon {
        let ally_pos = (target_side_idx, ally_idx);

        // if (ally !== source && !this.suppressingAbility(ally)) {
        let is_source = ally_pos == pokemon_pos;
        let is_suppressing = battle.suppressing_ability(Some(ally_pos));

        if !is_source && !is_suppressing {
            // Check abilities first
            let (has_sapsipper, has_goodasgold) = {
                let ally = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                (
                    ally.has_ability(&["sapsipper"]),
                    ally.has_ability(&["goodasgold"]),
                )
            };

            // if (ally.hasAbility('sapsipper')) {
            if has_sapsipper {
                // this.add('-immune', ally, '[from] ability: Sap Sipper');
                let ally_ident = {
                    let ally = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
                        Some(p) => p,
                        None => continue,
                    };
                    ally.get_slot()
                };
                battle.add(
                    "-immune",
                    &[
                        ally_ident.as_str().into(),
                        "[from] ability: Sap Sipper".into(),
                    ],
                );
                // continue;
                continue;
            }

            // if (ally.hasAbility('goodasgold')) {
            if has_goodasgold {
                // this.add('-immune', ally, '[from] ability: Good as Gold');
                let ally_ident = {
                    let ally = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
                        Some(p) => p,
                        None => continue,
                    };
                    ally.get_slot()
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

            // if (ally.volatiles['substitute'] && !move.infiltrates) continue;
            let ally = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
                Some(p) => p,
                None => continue,
            };
            let has_substitute = ally.volatiles.contains_key(&ID::from("substitute"));

            if has_substitute {
                let infiltrates = battle
                    .active_move
                    .as_ref()
                    .map(|m| m.infiltrates)
                    .unwrap_or(false);
                if !infiltrates {
                    continue;
                }
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

        let ally_mut = match battle.pokemon_at_mut(ally_pos.0, ally_pos.1) {
            Some(p) => p,
            None => continue,
        };

        if let Some((status, removed_nightmare)) = ally_mut.cure_status() {
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

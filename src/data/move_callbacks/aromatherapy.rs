//! Aromatherapy Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	aromatherapy: {
//! 		num: 312,
//! 		accuracy: true,
//! 		basePower: 0,
//! 		category: "Status",
//! 		isNonstandard: "Past",
//! 		name: "Aromatherapy",
//! 		pp: 5,
//! 		priority: 0,
//! 		flags: { snatch: 1, distance: 1, metronome: 1 },
//! 		onHit(target, source, move) {
//! 			this.add('-activate', source, 'move: Aromatherapy');
//! 			let success = false;
//! 			const allies = [...target.side.pokemon, ...target.side.allySide?.pokemon || []];
//! 			for (const ally of allies) {
//! 				if (ally !== source && !this.suppressingAbility(ally)) {
//! 					if (ally.hasAbility('sapsipper')) {
//! 						this.add('-immune', ally, '[from] ability: Sap Sipper');
//! 						continue;
//! 					}
//! 					if (ally.hasAbility('goodasgold')) {
//! 						this.add('-immune', ally, '[from] ability: Good as Gold');
//! 						continue;
//! 					}
//! 					if (ally.volatiles['substitute'] && !move.infiltrates) continue;
//! 				}
//! 				if (ally.cureStatus()) success = true;
//! 			}
//! 			return success;
//! 		},
//! 		target: "allyTeam",
//! 		type: "Grass",
//! 		zMove: { boost: { spe: 1 } },
//! 		contestType: "Clever",
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onHit callback for Aromatherapy
/// Cures status of all allies on the side
pub fn on_hit(
    battle: &mut Battle,
    target: (usize, usize),
    source: (usize, usize),
    move_id: &ID,
) -> MoveHandlerResult {
    // JavaScript: this.add('-activate', source, 'move: Aromatherapy');
    let source_id = if let Some(side) = battle.sides.get(source.0) {
        if let Some(pokemon) = side.pokemon.get(source.1) {
            format!("{}: {}", side.id_str(), pokemon.name)
        } else {
            String::from("unknown")
        }
    } else {
        String::from("unknown")
    };

    battle.add_log("-activate", &[&source_id, "move: Aromatherapy"]);

    // JavaScript: let success = false;
    let mut success = false;

    // JavaScript: const allies = [...target.side.pokemon, ...target.side.allySide?.pokemon || []];
    // For now, we'll just process target.side.pokemon (no allySide support yet)
    let (side_idx, _) = target;
    let pokemon_count = if let Some(side) = battle.sides.get(side_idx) {
        side.pokemon.len()
    } else {
        return MoveHandlerResult::False;
    };

    // JavaScript: for (const ally of allies) { ... }
    for poke_idx in 0..pokemon_count {
        let ally = (side_idx, poke_idx);

        // JavaScript: if (ally !== source && !this.suppressingAbility(ally)) { ... }
        // Check Sap Sipper and Good as Gold abilities for non-source allies
        if ally != source {
            let ability = if let Some(side) = battle.sides.get(ally.0) {
                if let Some(pokemon) = side.pokemon.get(ally.1) {
                    pokemon.ability.as_str().to_string()
                } else {
                    continue;
                }
            } else {
                continue;
            };

            // JavaScript: if (ally.hasAbility('sapsipper')) { ... }
            if ability == "sapsipper" {
                let ally_id = if let Some(side) = battle.sides.get(ally.0) {
                    if let Some(pokemon) = side.pokemon.get(ally.1) {
                        format!("{}: {}", side.id_str(), pokemon.name)
                    } else {
                        continue;
                    }
                } else {
                    continue;
                };
                battle.add_log("-immune", &[&ally_id, "[from] ability: Sap Sipper"]);
                continue;
            }

            // JavaScript: if (ally.hasAbility('goodasgold')) { ... }
            if ability == "goodasgold" {
                let ally_id = if let Some(side) = battle.sides.get(ally.0) {
                    if let Some(pokemon) = side.pokemon.get(ally.1) {
                        format!("{}: {}", side.id_str(), pokemon.name)
                    } else {
                        continue;
                    }
                } else {
                    continue;
                };
                battle.add_log("-immune", &[&ally_id, "[from] ability: Good as Gold"]);
                continue;
            }

            // JavaScript: if (ally.volatiles['substitute'] && !move.infiltrates) continue;
            let has_substitute = if let Some(side) = battle.sides.get(ally.0) {
                if let Some(pokemon) = side.pokemon.get(ally.1) {
                    pokemon.has_volatile(&ID::new("substitute"))
                } else {
                    continue;
                }
            } else {
                continue;
            };

            // TODO: Check move.infiltrates flag
            if has_substitute {
                continue;
            }
        }

        // JavaScript: if (ally.cureStatus()) success = true;
        let cured = battle.cure_status(ally);
        if cured {
            success = true;
        }
    }

    // JavaScript: return success;
    if success {
        MoveHandlerResult::True
    } else {
        MoveHandlerResult::False
    }
}

//! Frisk Ability - Reveals foe's held items
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	frisk: {
//! 		onStart(pokemon) {
//! 			for (const target of pokemon.foes()) {
//! 				if (target.item) {
//! 					this.add('-item', target, target.getItem().name, '[from] ability: Frisk', '[of] ' + pokemon, '[identify]');
//! 				}
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Frisk",
//! 		rating: 1.5,
//! 		num: 119,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(pokemon)
/// Reveals the opponent's held item
pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // for (const target of pokemon.foes())
    let foe_side_index = 1 - pokemon.side_index;

    // Collect foe items first to avoid borrow checker issues
    let mut foe_items: Vec<(String, String)> = Vec::new(); // (foe_name, item_name)

    if let Some(foe_side) = battle.sides.get(foe_side_index) {
        for foe in foe_side.pokemon.iter().filter(|p| p.is_active && !p.fainted) {
            // if (target.item)
            if !foe.item.is_empty() {
                foe_items.push((foe.name.clone(), foe.item.to_string()));
            }
        }
    }

    // Now add battle log messages
    for (foe_name, item_name) in foe_items {
        // this.add('-item', target, target.getItem().name, '[from] ability: Frisk', '[of] ' + pokemon, '[identify]');
        battle.add("-item", &[
            Arg::String(foe_name),
            Arg::String(item_name),
            Arg::Str("[from] ability: Frisk"),
            Arg::String(format!("[of] {}", pokemon.name)),
            Arg::Str("[identify]")
        ]);
    }

    AbilityHandlerResult::Undefined
}


//! Corrosive Gas Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Arg, Battle};
use crate::event::EventResult;

/// onHit(target, source) {
///     const item = target.takeItem(source);
///     if (item) {
///         this.add('-enditem', target, item.name, '[from] move: Corrosive Gas', `[of] ${source}`);
///     } else {
///         this.add('-fail', target, 'move: Corrosive Gas');
///     }
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Take the item from the target
    let item_data = {
        let target = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        target.take_item(Some(pokemon_pos))
    };

    let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    let source = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    if !item_data.id.is_empty() {
        // Get item name from Dex
        let item_name = battle.dex.items.get(item_data.id.as_str())
            .map(|i| i.name.clone())
            .unwrap_or_else(|| item_data.id.to_string());

        battle.add("-enditem", &[
            Arg::Pokemon(target),
            Arg::String(item_name),
            Arg::Str("[from] move: Corrosive Gas"),
            Arg::String(format!("[of] {}", source.name)),
        ]);
    } else {
        battle.add("-fail", &[Arg::Pokemon(target), Arg::Str("move: Corrosive Gas")]);
    }

    EventResult::Continue
}


//! Poltergeist Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTry(source, target) {
///     return !!target.item;
/// }
pub fn on_try(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Boolean(false),
    };

    // return !!target.item;
    let has_item = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Boolean(false),
        };
        target_pokemon.item != ID::from("")
    };

    EventResult::Boolean(has_item)
}

/// onTryHit(target, source, move) {
///     this.add('-activate', target, 'move: Poltergeist', this.dex.items.get(target.item).name);
/// }
pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    let target = target_pos;

    // this.add('-activate', target, 'move: Poltergeist', this.dex.items.get(target.item).name);
    let (target_arg, item_name) = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let target_arg = crate::battle::Arg::from(target_pokemon);

        let item_id = target_pokemon.item.clone();

        (target_arg, item_id)
    };

    if item_id == ID::from("") {
        return EventResult::Continue;
    }

    let item_name = {
        let item_data = match battle.dex.get_item_by_id(&item_id) {
            Some(item) => item,
            None => return EventResult::Continue,
        };
        item_data.name.clone()
    };

    battle.add("-activate", &[
        target_arg,
        "move: Poltergeist".into(),
        item_name.into(),
    ]);

    EventResult::Continue
}


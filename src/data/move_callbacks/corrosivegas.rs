//! Corrosive Gas Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
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
    // Get target and source
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };
    let source = pokemon_pos;

    // const item = target.takeItem(source);
    let item = {
        let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.take_item()
    };

    // if (item) {
    if let Some(item_id) = item {
        // this.add('-enditem', target, item.name, '[from] move: Corrosive Gas', `[of] ${source}`);
        let (target_arg, source_arg, item_name) = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let item_data = battle.dex.get_item_by_id(&item_id);
            let item_name = item_data.map(|i| i.name.clone()).unwrap_or_else(|| item_id.to_string());

            (Arg::from(target_pokemon), Arg::from(source_pokemon), item_name)
        };

        battle.add("-enditem", &[
            target_arg,
            item_name.into(),
            "[from] move: Corrosive Gas".into(),
            format!("[of] {}", source_arg).into(),
        ]);
    } else {
        // this.add('-fail', target, 'move: Corrosive Gas');
        let target_arg = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            Arg::from(target_pokemon)
        };

        battle.add("-fail", &[target_arg, "move: Corrosive Gas".into()]);
    }

    EventResult::Continue
}

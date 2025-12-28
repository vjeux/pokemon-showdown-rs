//! Pluck Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source, move) {
///     const item = target.getItem();
///     if (source.hp && item.isBerry && target.takeItem(source)) {
///         this.add('-enditem', target, item.name, '[from] stealeat', '[move] Pluck', `[of] ${source}`);
///         if (this.singleEvent('Eat', item, target.itemState, source, source, move)) {
///             this.runEvent('EatItem', source, source, move, item);
///             if (item.id === 'leppaberry') target.staleness = 'external';
///         }
///         if (item.onEat) source.ateBerry = true;
///     }
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const item = target.getItem();
    let item_id = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.item.clone()
    };

    // Check if item is empty
    if item_id == ID::from("") {
        return EventResult::Continue;
    }

    // if (source.hp && item.isBerry && target.takeItem(source)) {
    let source_hp = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.hp
    };

    if source_hp == 0 {
        return EventResult::Continue;
    }

    let is_berry = {
        let item_data = match battle.dex.get_item_by_id(&item_id) {
            Some(item) => item,
            None => return EventResult::Continue,
        };
        item_data.is_berry
    };

    if !is_berry {
        return EventResult::Continue;
    }

    let taken = {
        let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.take_item().is_some()
    };

    if !taken {
        return EventResult::Continue;
    }

    // this.add('-enditem', target, item.name, '[from] stealeat', '[move] Pluck', `[of] ${source}`);
    let (target_arg, item_name, source_arg) = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let target_arg = crate::battle::Arg::from(target_pokemon);

        let item_data = match battle.dex.get_item_by_id(&item_id) {
            Some(item) => item,
            None => return EventResult::Continue,
        };
        let item_name = item_data.name.clone();

        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let source_arg = crate::battle::Arg::from(source_pokemon);

        (target_arg, item_name, source_arg)
    };

    battle.add("-enditem", &[
        target_arg,
        item_name.into(),
        "[from] stealeat".into(),
        "[move] Pluck".into(),
        format!("[of] {}", source_arg).into(),
    ]);

    // if (this.singleEvent('Eat', item, target.itemState, source, source, move)) {
    let eat_result = battle.single_event("Eat", &item_id, Some(source), Some(source), Some(&ID::from("pluck")));

    if !matches!(eat_result, EventResult::Boolean(false)) {
        // this.runEvent('EatItem', source, source, move, item);
        battle.run_event("EatItem", source, Some(source), Some(&item_id), None);

        // if (item.id === 'leppaberry') target.staleness = 'external';
        if item_id == ID::from("leppaberry") {
            let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.staleness = Some("external".to_string());
        }
    }

    // if (item.onEat) source.ateBerry = true;
    // In JavaScript, onEat is a callback function for berries
    // Check if the item is a berry instead
    let has_on_eat = {
        let item_data = match battle.dex.get_item_by_id(&item_id) {
            Some(item) => item,
            None => return EventResult::Continue,
        };
        item_data.is_berry
    };

    if has_on_eat {
        let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.ate_berry = true;
    }

    EventResult::Continue
}


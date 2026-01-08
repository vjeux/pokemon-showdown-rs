//! Pluck Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Effect};
use crate::event::EventResult;
use crate::Pokemon;

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
pub fn on_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),  // First param is target
    source_pos: Option<(usize, usize)>,  // Second param is source
) -> EventResult {
    use crate::dex_data::ID;

    let target = target_pos;
    let source = match source_pos {
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
        let item_data = match battle.dex.items().get_by_id(&item_id) {
            Some(item) => item,
            None => return EventResult::Continue,
        };
        item_data.is_berry
    };

    if !is_berry {
        return EventResult::Continue;
    }

    // JS: target.takeItem(source)
    let taken = Pokemon::take_item(battle, target, Some(source)).is_some();

    if !taken {
        return EventResult::Continue;
    }

    // this.add('-enditem', target, item.name, '[from] stealeat', '[move] Pluck', `[of] ${source}`);
    let (target_arg, item_name, source_arg) = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let target_arg = target_pokemon.get_slot();

        let item_data = match battle.dex.items().get_by_id(&item_id) {
            Some(item) => item,
            None => return EventResult::Continue,
        };
        let item_name = item_data.name.clone();

        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let source_arg = source_pokemon.get_slot();

        (target_arg, item_name, source_arg)
    };

    battle.add(
        "-enditem",
        &[
            target_arg.into(),
            item_name.into(),
            "[from] stealeat".into(),
            "[move] Pluck".into(),
            format!("[of] {}", source_arg).into(),
        ],
    );

    // if (this.singleEvent('Eat', item, target.itemState, source, source, move)) {
    let eat_result = battle.single_event(
        "Eat",
        &crate::battle::Effect::item(item_id.clone()),
        None,
        Some(source),
        Some(source),
        Some(&Effect::move_(ID::from("pluck"))),
        None,
    );

    if !matches!(eat_result, EventResult::Boolean(false)) {
        // this.runEvent('EatItem', source, source, move, item);
        battle.run_event("EatItem", Some(crate::event::EventTarget::Pokemon(source)), Some(source), Some(&Effect::item(item_id.clone())), EventResult::Continue, false, false);

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
        let item_data = match battle.dex.items().get_by_id(&item_id) {
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

//! Bug Bite Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onHit(target, source, move) {
///     const item = target.getItem();
///     if (source.hp && item.isBerry && target.takeItem(source)) {
///         this.add('-enditem', target, item.name, '[from] stealeat', '[move] Bug Bite', `[of] ${source}`);
///         if (this.singleEvent('Eat', item, target.itemState, source, source, move)) {
///             this.runEvent('EatItem', source, source, move, item);
///             if (item.id === 'leppaberry') target.staleness = 'external';
///         }
///         if (item.onEat) source.ateBerry = true;
///     }
/// }
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),  // JS target (defender, has the berry)
    target_pos: Option<(usize, usize)>,  // JS source (attacker, eats the berry)
) -> EventResult {
    // Map Rust params to JS semantics:
    // pokemon_pos = JS target (defender)
    // target_pos = JS source (attacker)
    let target = pokemon_pos;  // Defender (has the berry)
    let source = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const item = target.getItem();
    let target_item = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.item.clone()
    };

    // if (source.hp && item.isBerry && target.takeItem(source)) {
    let source_hp = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.hp
    };

    if source_hp > 0 && !target_item.is_empty() {
        // Check if item is a berry
        let is_berry = if let Some(item_data) = battle.dex.items().get_by_id(&target_item) {
            // Check if item name ends with "berry" (simple heuristic)
            item_data.name.to_lowercase().ends_with("berry")
        } else {
            false
        };

        if is_berry {
            // target.takeItem(source)
            let taken_item = Pokemon::take_item(battle, target, Some(source));

            if let Some(item_id) = taken_item {
                // this.add('-enditem', target, item.name, '[from] stealeat', '[move] Bug Bite', `[of] ${source}`);
                let (target_ident, source_ident, item_name) = {
                    let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    let item_data = battle.dex.items().get_by_id(&item_id);
                    let item_name = item_data
                        .map(|i| i.name.clone())
                        .unwrap_or_else(|| item_id.to_string());

                    (
                        target_pokemon.get_slot(),
                        source_pokemon.get_slot(),
                        item_name,
                    )
                };

                battle.add(
                    "-enditem",
                    &[
                        target_ident.as_str().into(),
                        item_name.into(),
                        "[from] stealeat".into(),
                        "[move] Bug Bite".into(),
                        format!("[of] {}", source_ident).into(),
                    ],
                );

                // if (this.singleEvent('Eat', item, target.itemState, source, source, move)) {
                //     this.runEvent('EatItem', source, source, move, item);
                //     if (item.id === 'leppaberry') target.staleness = 'external';
                // }
                let eat_result = battle.single_event(
                    "Eat",
                    &crate::battle::Effect::item(item_id.clone()),
                    None,
                    Some(source),
                    Some(source),
                    None,
                    None,
                );
                if matches!(eat_result, EventResult::Boolean(true))
                    || matches!(eat_result, EventResult::Continue)
                {
                    battle.run_event("EatItem", Some(crate::event::EventTarget::Pokemon(source)), Some(source), None, EventResult::Continue, false, false);

                    // if (item.id === 'leppaberry') target.staleness = 'external';
                    if item_id.as_str() == "leppaberry" {
                        let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
                            Some(p) => p,
                            None => return EventResult::Continue,
                        };
                        target_pokemon.staleness = Some("external".to_string());
                    }
                }

                // if (item.onEat) source.ateBerry = true;
                // JavaScript checks `if (item.onEat)` which is a truthy check
                // `onEat: false` is falsy, so we need to check the value, not just key existence
                let has_on_eat = {
                    let item = battle.dex.items().get(item_id.as_str());
                    if let Some(item_data) = item {
                        item_data.extra.get("onEat").map_or(false, |v| {
                            !v.is_null() && v != &serde_json::Value::Bool(false)
                        })
                    } else {
                        false
                    }
                };
                if has_on_eat {
                    let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    source_pokemon.ate_berry = true;
                }
            }
        }
    }

    EventResult::Continue
}

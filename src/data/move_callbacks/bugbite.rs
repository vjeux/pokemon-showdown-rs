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
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    // Get target
    let target = match target_pos {
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
        let source_pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
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
            let taken_item = Pokemon::take_item(battle, target, Some(pokemon_pos));

            if let Some(item_id) = taken_item {
                // this.add('-enditem', target, item.name, '[from] stealeat', '[move] Bug Bite', `[of] ${source}`);
                let (target_ident, source_ident, item_name) = {
                    let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    let source_pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
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
                    &item_id,
                    Some(pokemon_pos),
                    Some(pokemon_pos),
                    None,
                );
                if matches!(eat_result, EventResult::Boolean(true))
                    || matches!(eat_result, EventResult::Continue)
                {
                    battle.run_event("EatItem", Some(pokemon_pos), Some(pokemon_pos), None, EventResult::Continue, false, false);

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
                // Check if item has onEat callback (simplified: just set ateBerry for berries)
                let source_pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                source_pokemon.ate_berry = true;
            }
        }
    }

    EventResult::Continue
}

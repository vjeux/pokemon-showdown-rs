//! Pickup Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onResidual(pokemon) {
///     if (pokemon.item) return;
///     const pickupTargets = this.getAllActive().filter(target => (
///         target.lastItem && target.usedItemThisTurn && pokemon.isAdjacent(target)
///     ));
///     if (!pickupTargets.length) return;
///     const randomTarget = this.sample(pickupTargets);
///     const item = randomTarget.lastItem;
///     randomTarget.lastItem = '';
///     this.add('-item', pokemon, this.dex.items.get(item), '[from] ability: Pickup');
///     pokemon.setItem(item);
/// }
pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    // if (pokemon.item) return;
    let has_item = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        !pokemon.item.is_empty()
    };

    if has_item {
        return EventResult::Continue;
    }

    // const pickupTargets = this.getAllActive().filter(target => (
    //     target.lastItem && target.usedItemThisTurn && pokemon.isAdjacent(target)
    // ));
    let mut pickup_targets: Vec<(usize, usize)> = Vec::new();

    for target_pos in battle.get_all_active(false) {
        // Get target pokemon to check conditions
        let (has_last_item, used_item) = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => continue,
            };

            (!target.last_item.is_empty(), target.used_item_this_turn)
        };

        // Check: target.lastItem && target.usedItemThisTurn && pokemon.isAdjacent(target)
        if has_last_item && used_item && battle.is_adjacent(pokemon_pos, target_pos) {
            pickup_targets.push(target_pos);
        }
    }

    // if (!pickupTargets.length) return;
    if pickup_targets.is_empty() {
        return EventResult::Continue;
    }

    // const randomTarget = this.sample(pickupTargets);
    let random_target_pos = match battle.sample(&pickup_targets) {
        Some(&pos) => pos,
        None => return EventResult::Continue,
    };

    // const item = randomTarget.lastItem;
    let item_id = {
        let random_target = match battle.pokemon_at(random_target_pos.0, random_target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        random_target.last_item.clone()
    };

    // randomTarget.lastItem = '';
    {
        let random_target = match battle.pokemon_at_mut(random_target_pos.0, random_target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        random_target.last_item = "".into();
    }

    // Get pokemon slot and item name for message
    let (pokemon_slot, item_name) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let item_name = battle.dex.items().get_by_id(&item_id)
            .map(|item| item.name.clone())
            .unwrap_or_else(|| item_id.to_string());

        (pokemon.get_slot(), item_name)
    };

    // this.add('-item', pokemon, this.dex.items.get(item), '[from] ability: Pickup');
    battle.add("-item", &[
        pokemon_slot.as_str().into(),
        item_name.into(),
        "[from] ability: Pickup".into(),
    ]);

    // pokemon.setItem(item);
    Pokemon::set_item(battle, pokemon_pos, ID::from(item_id), None, None);

    EventResult::Continue
}


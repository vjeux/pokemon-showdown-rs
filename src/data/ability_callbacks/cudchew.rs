//! Cud Chew Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::{Battle, Effect};
use crate::event::EventResult;

/// onEatItem(item, pokemon, source, effect) {
///     if (item.isBerry && (!effect || !['bugbite', 'pluck'].includes(effect.id))) {
///         this.effectState.berry = item;
///         this.effectState.counter = 2;
///         // This is needed in case the berry was eaten during residuals, preventing the timer from decreasing this turn
///         if (!this.queue.peek()) this.effectState.counter--;
///     }
/// }
pub fn on_eat_item(battle: &mut Battle, _item_id: Option<&str>, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, effect: Option<&Effect>) -> EventResult {
    let effect_id = effect.map(|e| e.id.as_str());
    

    // Get the item from battle.event.effect
    let item_id = match &battle.event {
        Some(event) => match &event.effect {
            Some(effect) => effect.id.clone(),
            None => return EventResult::Continue,
        },
        None => return EventResult::Continue,
    };

    // if (item.isBerry && (!effect || !['bugbite', 'pluck'].includes(effect.id)))
    let is_berry = if let Some(item_data) = battle.dex.items().get_by_id(&item_id) {
        item_data.is_berry
    } else {
        return EventResult::Continue;
    };

    if !is_berry {
        return EventResult::Continue;
    }

    // Check if effect is bugbite or pluck
    if let Some(eff_id) = effect_id {
        if eff_id == "bugbite" || eff_id == "pluck" {
            return EventResult::Continue;
        }
    }

    // this.effectState.berry = item;
    // this.effectState.counter = 2;
    let mut counter = 2;

    // if (!this.queue.peek()) this.effectState.counter--;
    if battle.queue.peek().is_none() {
        counter = 1;
    }

    // Store berry and counter in pokemon.ability_state.data
    let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    pokemon.ability_state.borrow_mut().berry = Some(item_id.to_string());
    pokemon.ability_state.borrow_mut().counter = Some(counter);

    EventResult::Continue
}

/// onResidual(pokemon) {
///     if (!this.effectState.berry || !pokemon.hp) return;
///     if (--this.effectState.counter <= 0) {
///         const item = this.effectState.berry;
///         this.add('-activate', pokemon, 'ability: Cud Chew');
///         this.add('-enditem', pokemon, item.name, '[eat]');
///         if (this.singleEvent('Eat', item, null, pokemon, null, null)) {
///             this.runEvent('EatItem', pokemon, null, null, item);
///         }
///         if (item.onEat) pokemon.ateBerry = true;
///         delete this.effectState.berry;
///         delete this.effectState.counter;
///     }
/// }
pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect: Option<&Effect>) -> EventResult {
    use crate::battle::Arg;
    use crate::dex_data::ID;

    // if (!this.effectState.berry || !pokemon.hp) return;
    let (berry_id, counter, hp) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let berry_str = match pokemon.ability_state.borrow().berry.as_ref() {
            Some(s) => s.clone(),
            None => return EventResult::Continue,
        };

        let counter = match pokemon.ability_state.borrow().counter {
            Some(c) => c,
            None => return EventResult::Continue,
        };

        (ID::from(berry_str.as_str()), counter, pokemon.hp)
    };

    if hp == 0 {
        return EventResult::Continue;
    }

    // if (--this.effectState.counter <= 0)
    let new_counter = counter - 1;

    if new_counter > 0 {
        // Update counter
        let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.ability_state.borrow_mut().counter = Some(new_counter);
        return EventResult::Continue;
    }

    // counter <= 0, eat the berry
    let item_name = if let Some(item_data) = battle.dex.items().get_by_id(&berry_id) {
        item_data.name.clone()
    } else {
        berry_id.to_string()
    };

    let pokemon_slot = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    // this.add('-activate', pokemon, 'ability: Cud Chew');
    battle.add("-activate", &[
        Arg::String(pokemon_slot.clone()),
        Arg::Str("ability: Cud Chew"),
    ]);

    // this.add('-enditem', pokemon, item.name, '[eat]');
    battle.add("-enditem", &[
        Arg::String(pokemon_slot),
        Arg::String(item_name),
        Arg::Str("[eat]"),
    ]);

    // if (this.singleEvent('Eat', item, null, pokemon, null, null))
    let item_effect = battle.make_item_effect(&berry_id);
    let eat_result = battle.single_event(
        "Eat",
        &item_effect,
        None,
        Some(pokemon_pos),
        None,
        None,
        None,
    );

    if eat_result.is_not_fail() {
        // this.runEvent('EatItem', pokemon, null, null, item);
        battle.run_event("EatItem", Some(crate::event::EventTarget::Pokemon(pokemon_pos)), None, Some(&item_effect), EventResult::Continue, false, false);
    }

    // if (item.onEat) pokemon.ateBerry = true;
    // JavaScript checks `if (item.onEat)` which is a truthy check
    // `onEat: false` is falsy, so we need to check the value, not just key existence
    if let Some(item_data) = battle.dex.items().get_by_id(&berry_id) {
        let on_eat_is_truthy = item_data.extra.get("onEat").map_or(false, |v| {
            !v.is_null() && v != &serde_json::Value::Bool(false)
        });
        if on_eat_is_truthy {
            if let Some(pokemon) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                pokemon.ate_berry = true;
            }
        }
    }

    // delete this.effectState.berry;
    // delete this.effectState.counter;
    if let Some(pokemon) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        pokemon.ability_state.borrow_mut().berry = None;
        pokemon.ability_state.borrow_mut().counter = None;
    }

    EventResult::Continue
}


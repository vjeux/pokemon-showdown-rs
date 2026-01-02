//! Berry Juice Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onUpdate(pokemon) {
///     if (pokemon.hp <= pokemon.maxhp / 2) {
///         if (this.runEvent('TryHeal', pokemon, null, this.effect, 20) && pokemon.useItem()) {
///             this.heal(20);
///         }
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.hp <= pokemon.maxhp / 2) {
    //     if (this.runEvent('TryHeal', pokemon, null, this.effect, 20) && pokemon.useItem()) {
    //         this.heal(20);
    //     }
    // }

    // Phase 1: Check HP threshold
    let should_heal = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.hp <= pokemon.maxhp / 2
    };

    if should_heal {
        // Phase 2: Run TryHeal event
        let try_heal_result = battle.run_event("TryHeal", Some(pokemon_pos), None, None, Some(20));

        if try_heal_result.is_some() {
            // Phase 3: Try to use item
            let used_item = {
                let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                pokemon_mut.use_item(None, None).is_some()
            };

            if used_item {
                // Phase 4: Heal
                battle.heal(20, Some(pokemon_pos), None, None);
            }
        }
    }

    EventResult::Continue
}

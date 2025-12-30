//! Mental Herb Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onUpdate(pokemon) {
///     const conditions = ['attract', 'taunt', 'encore', 'torment', 'disable', 'healblock'];
///     for (const firstCondition of conditions) {
///         if (pokemon.volatiles[firstCondition]) {
///             if (!pokemon.useItem()) return;
///             for (const secondCondition of conditions) {
///                 pokemon.removeVolatile(secondCondition);
///                 if (firstCondition === 'attract' && secondCondition === 'attract') {
///                     this.add('-end', pokemon, 'move: Attract', '[from] item: Mental Herb');
///                 }
///             }
///             return;
///         }
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // const conditions = ['attract', 'taunt', 'encore', 'torment', 'disable', 'healblock'];
    let conditions = ["attract", "taunt", "encore", "torment", "disable", "healblock"];

    // for (const firstCondition of conditions) {
    //     if (pokemon.volatiles[firstCondition]) {
    let first_condition_found = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        conditions.iter().find(|&&cond| pokemon.has_volatile(&crate::dex_data::ID::new(cond))).copied()
    };

    if let Some(_first_condition) = first_condition_found {
        // if (!pokemon.useItem()) return;
        let used_item = {
            let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_mut.use_item().is_some()
        };

        if !used_item {
            return EventResult::Continue;
        }

        // for (const secondCondition of conditions) {
        //     pokemon.removeVolatile(secondCondition);
        for &condition in &conditions {
            let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_mut.remove_volatile(&crate::dex_data::ID::new(condition));
        }

        return EventResult::Continue;
    }

    EventResult::Continue
}

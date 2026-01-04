//! Silvally Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onType
/// JavaScript source (data/conditions.ts):
/// ```js
/// onTypePriority: 1,
/// onType(types, pokemon) {
///     if (pokemon.transformed || pokemon.ability !== 'rkssystem' && this.gen >= 8) return types;
///     let type: string | undefined = 'Normal';
///     if (pokemon.ability === 'rkssystem') {
///         type = pokemon.getItem().onMemory;
///         if (!type) {
///             type = 'Normal';
///         }
///     }
///     return [type];
/// }
/// ```
pub fn on_type(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // if (pokemon.transformed || pokemon.ability !== 'rkssystem' && this.gen >= 8) return types;
    let (transformed, has_rkssystem) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon.transformed, pokemon.has_ability(battle, &["rkssystem"]))
    };

    if transformed || (!has_rkssystem && battle.gen >= 8) {
        return EventResult::Continue;
    }

    // let type: string | undefined = 'Normal';
    let mut type_name = "normal".to_string();

    // if (pokemon.ability === 'rkssystem')
    if has_rkssystem {
        // type = pokemon.getItem().onMemory;
        let memory_type = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let item_id = pokemon.get_item();
            battle.dex.items().get_by_id(item_id)
                .and_then(|item| item.on_memory.clone())
        };

        if let Some(mt) = memory_type {
            type_name = mt;
        }
        // if (!type) { type = 'Normal'; } is already handled by initialization
    }

    // return [type];
    EventResult::Types(vec![type_name])
}


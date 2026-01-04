//! Arceus Condition
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
///     if (pokemon.transformed || pokemon.ability !== 'multitype' && this.gen >= 8) return types;
///     let type: string | undefined = 'Normal';
///     if (pokemon.ability === 'multitype') {
///         type = pokemon.getItem().onPlate;
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
    // if (pokemon.transformed || pokemon.ability !== 'multitype' && this.gen >= 8) return types;
    let (transformed, has_multitype) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon.transformed, pokemon.has_ability(battle, &["multitype"]))
    };

    if transformed || (!has_multitype && battle.gen >= 8) {
        return EventResult::Continue;
    }

    // let type: string | undefined = 'Normal';
    let mut type_name = "normal".to_string();

    // if (pokemon.ability === 'multitype')
    if has_multitype {
        // type = pokemon.getItem().onPlate;
        let plate_type = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let item_id = pokemon.get_item();
            battle.dex.items().get_by_id(item_id)
                .and_then(|item| item.on_plate.clone())
        };

        if let Some(pt) = plate_type {
            type_name = pt;
        }
        // if (!type) { type = 'Normal'; } is already handled by initialization
    }

    // return [type];
    EventResult::Types(vec![type_name])
}


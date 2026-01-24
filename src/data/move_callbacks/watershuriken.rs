//! Water Shuriken Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target, move) {
///     if (pokemon.species_id.as_str() === 'Greninja-Ash' && pokemon.hasAbility('battlebond') &&
///         !pokemon.transformed) {
///         return move.basePower + 5;
///     }
///     return move.basePower;
/// }
pub fn base_power_callback(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    // Get the pokemon
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // Get the active move
    let active_move = match &battle.active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // if (pokemon.species_id.as_str() === 'Greninja-Ash' && pokemon.hasAbility('battlebond') && !pokemon.transformed)
    if pokemon.species_id == ID::from("greninjaash")
        && pokemon.has_ability(battle, &["battlebond"])
        && !pokemon.transformed
    {
        return EventResult::Number(active_move.borrow().base_power + 5);
    }

    EventResult::Number(active_move.borrow().base_power)
}

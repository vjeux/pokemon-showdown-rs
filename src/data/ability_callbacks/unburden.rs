//! Unburden Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAfterUseItem(item, pokemon) {
///     if (pokemon !== this.effectState.target) return;
///     pokemon.addVolatile('unburden');
/// }
pub fn on_after_use_item(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::Pokemon;

    // pokemon.addVolatile('unburden');
    Pokemon::add_volatile(
        battle,
        pokemon_pos,
        crate::dex_data::ID::from("unburden"),
        None,
        None,
        None,
    );

    EventResult::Continue
}

/// onTakeItem(item, pokemon) {
///     pokemon.addVolatile('unburden');
/// }
pub fn on_take_item(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>) -> EventResult {
    use crate::Pokemon;

    // pokemon.addVolatile('unburden');
    Pokemon::add_volatile(
        battle,
        pokemon_pos,
        crate::dex_data::ID::from("unburden"),
        None,
        None,
        None,
    );

    EventResult::Continue
}

/// onEnd(pokemon) {
///     pokemon.removeVolatile('unburden');
/// }
pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::Pokemon;

    // pokemon.removeVolatile('unburden');
    Pokemon::remove_volatile(battle, pokemon_pos, &crate::dex_data::ID::from("unburden"));

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onModifySpe(spe, pokemon) {
    ///     if (!pokemon.item && !pokemon.ignoringAbility()) {
    ///         return this.chainModify(2);
    ///     }
    /// }
    pub fn on_modify_spe(battle: &mut Battle, _spe: i32, pokemon_pos: (usize, usize)) -> EventResult {
        // if (!pokemon.item && !pokemon.ignoringAbility())
        let (has_no_item, ignoring_ability) = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            (pokemon.item.is_empty(), pokemon.ignoring_ability(battle))
        };

        if has_no_item && !ignoring_ability {
            // return this.chainModify(2);
            return EventResult::Number(2);
        }

        EventResult::Continue
    }
}

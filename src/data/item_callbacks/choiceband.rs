//! Choice Band Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onStart(pokemon) {
///     if (pokemon.volatiles['choicelock']) {
///         this.debug('removing choicelock');
///     }
///     pokemon.removeVolatile('choicelock');
/// }
pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    // if (pokemon.volatiles['choicelock']) {
    //     this.debug('removing choicelock');
    // }
    // pokemon.removeVolatile('choicelock');

    let pokemon_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let has_choicelock = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.has_volatile(&ID::new("choicelock"))
    };

    if has_choicelock {
        battle.debug("removing choicelock");
    }

    Pokemon::remove_volatile(battle, pokemon_pos, &ID::new("choicelock"));

    EventResult::Continue
}

/// onModifyMove(move, pokemon) {
///     pokemon.addVolatile('choicelock');
/// }
pub fn on_modify_move(battle: &mut Battle, pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // pokemon.addVolatile('choicelock');
    Pokemon::add_volatile(battle, pokemon_pos, ID::new("choicelock"), None, None, None, None);

    EventResult::Continue
}

/// onModifyAtk(atk, pokemon) {
///     if (pokemon.volatiles['dynamax']) return;
///     return this.chainModify(1.5);
/// }
pub fn on_modify_atk(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.volatiles['dynamax']) return;
    let has_dynamax = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.has_volatile(&crate::dex_data::ID::new("dynamax"))
    };

    if has_dynamax {
        return EventResult::Continue;
    }

    // return this.chainModify(1.5);
    battle.chain_modify(1.5);
    EventResult::Continue
}

//! Mustrecharge Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::battle::Arg;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onBeforeMove
/// JavaScript source (data/conditions.ts):
/// ```js
/// onBeforeMovePriority: 11,
/// onBeforeMove(pokemon) {
///     this.add('cant', pokemon, 'recharge');
///     pokemon.removeVolatile('mustrecharge');
///     pokemon.removeVolatile('truant');
///     return null;
/// }
/// ```
pub fn on_before_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // this.add('cant', pokemon, 'recharge');
    let pokemon_ident = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add("cant", &[Arg::String(pokemon_ident), Arg::Str("recharge")]);

    // pokemon.removeVolatile('mustrecharge');
    let mustrecharge_id = ID::from("mustrecharge");
    crate::pokemon::Pokemon::remove_volatile(battle, pokemon_pos, &mustrecharge_id);

    // pokemon.removeVolatile('truant');
    let truant_id = ID::from("truant");
    crate::pokemon::Pokemon::remove_volatile(battle, pokemon_pos, &truant_id);

    // return null;
    EventResult::Null
}

/// onStart
/// JavaScript source (data/conditions.ts):
/// ```js
/// onStart(pokemon) {
///     this.add('-mustrecharge', pokemon);
/// }
/// ```
pub fn on_start(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // this.add('-mustrecharge', pokemon);
    let pokemon_ident = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add("-mustrecharge", &[Arg::String(pokemon_ident)]);

    EventResult::Continue
}


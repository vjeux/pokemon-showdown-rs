//! Detect Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onPrepareHit(pokemon) {
///     return !!this.queue.willAct() && this.runEvent('StallMove', pokemon);
/// }
pub fn on_prepare_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // return !!this.queue.willAct() && this.runEvent('StallMove', pokemon);
    let pokemon = pokemon_pos;

    // !!this.queue.willAct()
    let will_act = battle.queue.will_act().is_some();

    if !will_act {
        return EventResult::Boolean(false);
    }

    // this.runEvent('StallMove', pokemon)
    let stall_result = battle.run_event("StallMove", Some(crate::event::EventTarget::Pokemon(pokemon)), None, None, EventResult::Continue, false, false);

    // Convert stall_result to boolean: Boolean(true/false) or Number(!=0) means success
    let stall_success = match stall_result {
        EventResult::Boolean(b) => b,
        EventResult::Number(n) => n != 0,
        _ => false,
    };

    EventResult::Boolean(will_act && stall_success)
}

/// onHit(pokemon) {
///     pokemon.addVolatile('stall');
/// }
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;
    use crate::pokemon::Pokemon;

    // pokemon.addVolatile('stall');
    // Use battle.add_volatile_to_pokemon to properly set duration from dex.conditions
    Pokemon::add_volatile(battle, pokemon_pos, ID::from("stall"), None, None, None, None);

    EventResult::Continue
}

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
    let stall_result = battle.run_event("StallMove", Some(pokemon), None, None, None);

    // return !!this.queue.willAct() && this.runEvent('StallMove', pokemon);
    EventResult::Boolean(will_act && stall_result.unwrap_or(0) != 0)
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

    // pokemon.addVolatile('stall');
    // Use battle.add_volatile_to_pokemon to properly set duration from dex.conditions
    battle.add_volatile_to_pokemon(pokemon_pos, ID::from("stall"), None);

    EventResult::Continue
}

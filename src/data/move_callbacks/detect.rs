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
pub fn on_prepare_hit(battle: &mut Battle, pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // return !!this.queue.willAct() && this.runEvent('StallMove', pokemon);
    let pokemon = pokemon_pos;

    // !!this.queue.willAct()
    let will_act = battle.queue.will_act();

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
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    // pokemon.addVolatile('stall');
    let pokemon = pokemon_pos;

    let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };
    pokemon_pokemon.add_volatile(ID::from("stall"));

    EventResult::Continue
}


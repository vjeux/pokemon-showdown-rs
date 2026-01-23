//! Belch Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDisableMove(pokemon) {
///     if (!pokemon.ateBerry) pokemon.disableMove('belch');
/// }
pub fn on_disable_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::debug_elog;

    debug_elog!("[BELCH] on_disable_move called for pokemon at {:?}", pokemon_pos);

    // if (!pokemon.ateBerry) pokemon.disableMove('belch');
    let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => {
            debug_elog!("[BELCH] Pokemon not found at {:?}", pokemon_pos);
            return EventResult::Continue;
        }
    };

    debug_elog!("[BELCH] Pokemon {} ate_berry={}", pokemon.set.species, pokemon.ate_berry);

    if !pokemon.ate_berry {
        debug_elog!("[BELCH] Disabling belch for {}", pokemon.set.species);
        pokemon.disable_move("belch", false, None);
    }

    EventResult::Continue
}

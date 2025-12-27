//! Swallow Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTry(source, target, move) {
///     if (move.sourceEffect === 'snatch') return;
///     return !!source.volatiles['stockpile'];
/// }
pub fn on_try(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onHit(pokemon) {
///     const layers = pokemon.volatiles['stockpile']?.layers || 1;
///     const healAmount = [0.25, 0.5, 1];
///     const success = !!this.heal(this.modify(pokemon.maxhp, healAmount[layers - 1]));
///     if (!success) this.add('-fail', pokemon, 'heal');
///     pokemon.removeVolatile('stockpile');
///     return success || this.NOT_FAIL;
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}


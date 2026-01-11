//! Clear Smog Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target) {
///     target.clearBoosts();
///     this.add('-clearboost', target);
/// }
/// JavaScript signature: onHit(target, source, move) - TARGET FIRST
/// dispatch_on_hit passes: (target_pos, source_pos)
pub fn on_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
) -> EventResult {
    // Get the target (first parameter)
    let target = target_pos;

    // target.clearBoosts();
    {
        let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.clear_boosts();
    }

    // this.add('-clearboost', target);
    let target_ident = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.get_slot()
    };

    battle.add("-clearboost", &[target_ident.as_str().into()]);

    EventResult::Continue
}

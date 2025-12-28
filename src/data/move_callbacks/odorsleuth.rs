//! Odor Sleuth Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target) {
///     if (target.volatiles['miracleeye']) return false;
/// }
pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    let target = target_pos;

    // if (target.volatiles['miracleeye']) return false;
    let has_miracleeye = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.volatiles.contains_key(&ID::from("miracleeye"))
    };

    if has_miracleeye {
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}


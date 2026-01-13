//! Sappy Seed Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onHit(target, source) {
///     if (target.hasType('Grass')) return null;
///     target.addVolatile('leechseed', source);
/// }
pub fn on_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),  // First param is target
    source_pos: Option<(usize, usize)>,  // Second param is source
) -> EventResult {
    let target = target_pos;
    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (target.hasType('Grass')) return null;
    let has_grass = {
        let pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.has_type(battle, "Grass")
    };

    if has_grass {
        // return null; - prevents further execution
        return EventResult::Null;
    }

    // target.addVolatile('leechseed', source);
    Pokemon::add_volatile(battle, target, ID::from("leechseed"), Some(source), None, None, None);

    EventResult::Continue
}

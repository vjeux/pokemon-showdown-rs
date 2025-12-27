//! Sappy Seed Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source) {
///     if (target.hasType('Grass')) return null;
///     target.addVolatile('leechseed', source);
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (target.hasType('Grass')) return null;
    let has_grass = battle.has_type(target, "Grass");

    if has_grass {
        return EventResult::Null;
    }

    // target.addVolatile('leechseed', source);
    battle.add_volatile(&ID::from("leechseed"), target, Some(source), None);

    EventResult::Continue
}


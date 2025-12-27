//! Thousand Waves Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source, move) {
///     if (source.isActive) target.addVolatile('trapped', source, move, 'trapper');
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Check if source is active before trapping
    let source_active = if let Some(source) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        source.is_active
    } else {
        return EventResult::Continue;
    };

    if source_active {
        // Add trapped volatile to target
        if let Some(target) = battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            target.add_volatile(crate::dex_data::ID::new("trapped"));
        }
    }

    EventResult::Continue
}


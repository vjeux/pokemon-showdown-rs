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
pub fn on_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),  // First param is target
    source_pos: Option<(usize, usize)>,  // Second param is source
) -> EventResult {
    use crate::dex_data::ID;

    let target = target_pos;
    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (source.isActive) target.addVolatile('trapped', source, move, 'trapper');
    let source_is_active = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.is_active
    };

    if source_is_active {
        // target.addVolatile('trapped', source, move, 'trapper');
        // JavaScript: target.addVolatile('trapped', source, move, 'trapper')
        // ✅ NOW PASSING: source_pos = Some(source), source_effect = Some("thousandwaves"), linked_status = Some("trapper")
        use crate::pokemon::Pokemon;
        let move_effect = battle.make_move_effect(&ID::new("thousandwaves"));
        Pokemon::add_volatile(
            battle,
            target,
            ID::from("trapped"),
            Some(source),
            Some(&move_effect),
            Some(ID::from("trapper")),
            None,
        );
    }

    EventResult::Continue
}

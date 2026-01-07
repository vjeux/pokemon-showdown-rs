//! Thousand Waves Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Effect};
use crate::event::EventResult;

/// onHit(target, source, move) {
///     if (source.isActive) target.addVolatile('trapped', source, move, 'trapper');
/// }
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    let source = pokemon_pos;
    let target = match target_pos {
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
        Pokemon::add_volatile(
            battle,
            target,
            ID::from("trapped"),
            Some(source),
            Some(&Effect::move_(ID::new("thousandwaves"))),
            Some(ID::from("trapper")),
            None,
        );
    }

    EventResult::Continue
}

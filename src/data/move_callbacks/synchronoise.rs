//! Synchronoise Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryImmunity(target, source) {
///     return target.hasType(source.getTypes());
/// }
pub fn on_try_immunity(
    battle: &mut Battle,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // return target.hasType(source.getTypes());
    let source_types = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.get_types(battle, false)
    };

    let has_shared_type = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        source_types.iter().any(|type_name| target_pokemon.has_type(battle, type_name))
    };

    EventResult::Boolean(has_shared_type)
}

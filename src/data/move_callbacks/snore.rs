//! Snore Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTry(source) {
///     return source.status === 'slp' || source.hasAbility('comatose');
/// }
pub fn on_try(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    // onTry(source) {
    //     return source.status === 'slp' || source.hasAbility('comatose');
    // }
    let source = source_pos;

    // return source.status === 'slp' || source.hasAbility('comatose');
    let has_sleep_or_comatose = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let has_sleep = source_pokemon.status == ID::from("slp");
        let has_comatose = source_pokemon.has_ability(battle, &["comatose"]);

        has_sleep || has_comatose
    };

    EventResult::Boolean(has_sleep_or_comatose)
}

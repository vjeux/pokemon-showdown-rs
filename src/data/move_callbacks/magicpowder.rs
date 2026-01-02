//! Magic Powder Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onHit(target) {
///     if (target.getTypes().join() === 'Psychic' || !target.setType('Psychic')) return false;
///     this.add('-start', target, 'typechange', 'Psychic');
/// }
pub fn on_hit(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (target.getTypes().join() === 'Psychic' || !target.setType('Psychic')) return false;
    let types_string = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.get_types(battle, false).join("")
    };

    // Try to set the type and check if it succeeded
    let set_type_succeeded = Pokemon::set_type(battle, target, vec!["Psychic".to_string()], false);

    // Check if it failed (either already Psychic or setType returned false)
    if types_string == "Psychic" || !set_type_succeeded {
        return EventResult::Boolean(false);
    }

    // this.add('-start', target, 'typechange', 'Psychic');
    let target_arg = {
        let pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,

            None => return EventResult::Continue,
        };

        pokemon.get_slot()
    };
    battle.add(
        "-start",
        &[target_arg.into(), "typechange".into(), "Psychic".into()],
    );

    EventResult::Continue
}

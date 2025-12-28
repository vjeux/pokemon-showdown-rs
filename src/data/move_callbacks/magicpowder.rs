//! Magic Powder Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target) {
///     if (target.getTypes().join() === 'Psychic' || !target.setType('Psychic')) return false;
///     this.add('-start', target, 'typechange', 'Psychic');
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

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
        target_pokemon.get_types(false).join("")
    };

    if types_string == "Psychic" {
        return EventResult::Boolean(false);
    }

    {
        let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.set_type(vec!["Psychic".to_string()]);
    }

    // this.add('-start', target, 'typechange', 'Psychic');
    let target_arg = {

        let pokemon = match battle.pokemon_at(target.0, target.1) {

            Some(p) => p,

            None => return EventResult::Continue,

        };

        pokemon.get_slot()

    };
    battle.add("-start", &[target_arg, "typechange".into(), "Psychic".into()]);

    EventResult::Continue
}


//! Forest's Curse Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target) {
///     if (target.hasType('Grass')) return false;
///     if (!target.addType('Grass')) return false;
///     this.add('-start', target, 'typeadd', 'Grass', '[from] move: Forest\'s Curse');
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

    // if (target.hasType('Grass')) return false;
    let has_grass_type = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.has_type("grass")
    };

    if has_grass_type {
        return EventResult::Boolean(false);
    }

    // if (!target.addType('Grass')) return false;
    {
        let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.add_type(String::from("Grass"));
    }

    // this.add('-start', target, 'typeadd', 'Grass', '[from] move: Forest\'s Curse');
    let target_ident = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.get_slot()
    };

    battle.add(
        "-start",
        &[
            target_ident.as_str().into(),
            "typeadd".into(),
            "Grass".into(),
            "[from] move: Forest's Curse".into(),
        ],
    );

    EventResult::Continue
}

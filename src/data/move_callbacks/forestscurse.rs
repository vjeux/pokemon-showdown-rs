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
    target_pos: (usize, usize),          // target - the Pokemon that gets Grass type added
    _source_pos: Option<(usize, usize)>, // source - the move user (not used)
) -> EventResult {
    // JavaScript: onHit(target) - target is the Pokemon that gets Grass type added
    // The dispatcher passes (target, source), so we use the first parameter
    let target = target_pos;

    // if (target.hasType('Grass')) return false;
    let has_grass_type = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.has_type(battle, "grass")
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

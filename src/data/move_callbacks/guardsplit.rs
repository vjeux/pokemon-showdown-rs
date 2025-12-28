//! Guard Split Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source) {
///     const newdef = Math.floor((target.storedStats.def + source.storedStats.def) / 2);
///     target.storedStats.def = newdef;
///     source.storedStats.def = newdef;
///     const newspd = Math.floor((target.storedStats.spd + source.storedStats.spd) / 2);
///     target.storedStats.spd = newspd;
///     source.storedStats.spd = newspd;
///     this.add('-activate', source, 'move: Guard Split', `[of] ${target}`);
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const newdef = Math.floor((target.storedStats.def + source.storedStats.def) / 2);
    let new_def = {
        let target_def = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.stored_stats.def
        };

        let source_def = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.stored_stats.def
        };

        (target_def + source_def) / 2
    };

    // target.storedStats.def = newdef;
    // source.storedStats.def = newdef;
    {
        let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.stored_stats.def = new_def;
    }
    {
        let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.stored_stats.def = new_def;
    }

    // const newspd = Math.floor((target.storedStats.spd + source.storedStats.spd) / 2);
    let new_spd = {
        let target_spd = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.stored_stats.spd
        };

        let source_spd = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.stored_stats.spd
        };

        (target_spd + source_spd) / 2
    };

    // target.storedStats.spd = newspd;
    // source.storedStats.spd = newspd;
    {
        let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.stored_stats.spd = new_spd;
    }
    {
        let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.stored_stats.spd = new_spd;
    }

    // this.add('-activate', source, 'move: Guard Split', `[of] ${target}`);
    let source_arg = {
        let pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        crate::battle::Arg::from(pokemon)
    };
    let target_arg = {
        let pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        crate::battle::Arg::from(pokemon)
    };
    battle.add("-activate", &[source_arg, "move: Guard Split".into(), "[of]".into(), target_arg]);

    EventResult::Continue
}


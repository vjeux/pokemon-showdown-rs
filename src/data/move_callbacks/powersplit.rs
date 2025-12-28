//! Power Split Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source) {
///     const newatk = Math.floor((target.storedStats.atk + source.storedStats.atk) / 2);
///     target.storedStats.atk = newatk;
///     source.storedStats.atk = newatk;
///     const newspa = Math.floor((target.storedStats.spa + source.storedStats.spa) / 2);
///     target.storedStats.spa = newspa;
///     source.storedStats.spa = newspa;
///     this.add('-activate', source, 'move: Power Split', `[of] ${target}`);
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const newatk = Math.floor((target.storedStats.atk + source.storedStats.atk) / 2);
    // target.storedStats.atk = newatk;
    // source.storedStats.atk = newatk;
    // const newspa = Math.floor((target.storedStats.spa + source.storedStats.spa) / 2);
    // target.storedStats.spa = newspa;
    // source.storedStats.spa = newspa;
    let (newatk, newspa) = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let newatk = (target_pokemon.stored_stats.atk + source_pokemon.stored_stats.atk) / 2;
        let newspa = (target_pokemon.stored_stats.spa + source_pokemon.stored_stats.spa) / 2;

        (newatk, newspa)
    };

    // Set target stats
    let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };
    target_pokemon.stored_stats.atk = newatk;
    target_pokemon.stored_stats.spa = newspa;

    // Set source stats
    let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };
    source_pokemon.stored_stats.atk = newatk;
    source_pokemon.stored_stats.spa = newspa;

    // this.add('-activate', source, 'move: Power Split', `[of] ${target}`);
    let (source_arg, target_arg) = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (source_pokemon.get_slot(), target_pokemon.get_slot())
    };

    battle.add("-activate", &[
        source_arg,
        "move: Power Split".into(),
        format!("[of] {}", target_arg).into(),
    ]);

    EventResult::Continue
}


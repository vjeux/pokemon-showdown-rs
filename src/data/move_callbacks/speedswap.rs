//! Speed Swap Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source) {
///     const targetSpe = target.storedStats.spe;
///     target.storedStats.spe = source.storedStats.spe;
///     source.storedStats.spe = targetSpe;
///     this.add('-activate', source, 'move: Speed Swap', `[of] ${target}`);
/// }
pub fn on_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),  // First param is target
    source_pos: Option<(usize, usize)>,  // Second param is source
) -> EventResult {
    // onHit(target, source) {
    //     const targetSpe = target.storedStats.spe;
    //     target.storedStats.spe = source.storedStats.spe;
    //     source.storedStats.spe = targetSpe;
    //     this.add('-activate', source, 'move: Speed Swap', `[of] ${target}`);
    // }
    let target = target_pos;
    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const targetSpe = target.storedStats.spe;
    // target.storedStats.spe = source.storedStats.spe;
    // source.storedStats.spe = targetSpe;
    let target_spe = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.stored_stats.spe
    };
    let source_spe = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.stored_stats.spe
    };
    {
        let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.stored_stats.spe = source_spe;
        // Update speed field to match stored_stats.spe (same pattern as Transform fix)
        target_pokemon.speed = target_pokemon.stored_stats.spe as i32;
    }
    {
        let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.stored_stats.spe = target_spe;
        // Update speed field to match stored_stats.spe (same pattern as Transform fix)
        source_pokemon.speed = source_pokemon.stored_stats.spe as i32;
    }

    // this.add('-activate', source, 'move: Speed Swap', `[of] ${target}`);
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

    battle.add(
        "-activate",
        &[
            source_arg.into(),
            "move: Speed Swap".into(),
            format!("[of] {}", target_arg).into(),
        ],
    );

    EventResult::Continue
}

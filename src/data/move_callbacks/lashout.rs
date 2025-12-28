//! Lash Out Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, source) {
///     if (source.statsLoweredThisTurn) {
///         this.debug('lashout buff');
///         return this.chainModify(2);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, base_power: i32, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let source = pokemon_pos;

    // if (source.statsLoweredThisTurn) {
    let stats_lowered_this_turn = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.stats_lowered_this_turn
    };

    if stats_lowered_this_turn {
        //     this.debug('lashout buff');
        battle.debug("lashout buff");

        //     return this.chainModify(2);
        return EventResult::Number(battle.chain_modify(2.0 as f32));
    }

    EventResult::Continue
}


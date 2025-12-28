//! Fusion Flare Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, pokemon) {
///     if (this.lastSuccessfulMoveThisTurn === 'fusionbolt') {
///         this.debug('double power');
///         return this.chainModify(2);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, _base_power: i32, _pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    // if (this.lastSuccessfulMoveThisTurn === 'fusionbolt') {
    if let Some(ref last_move) = battle.last_successful_move_this_turn {
        if *last_move == ID::from("fusionbolt") {
            // this.debug('double power');
            battle.debug("double power");

            // return this.chainModify(2);
            return EventResult::Number(battle.chain_modify(2.0 as f32));
        }
    }

    EventResult::Continue
}


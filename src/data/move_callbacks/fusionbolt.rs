//! Fusion Bolt Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, pokemon) {
///     if (this.lastSuccessfulMoveThisTurn === 'fusionflare') {
///         this.debug('double power');
///         return this.chainModify(2);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, base_power: i32, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    // if (this.lastSuccessfulMoveThisTurn === 'fusionflare') {
    if let Some(ref last_move) = battle.last_successful_move_this_turn {
        if *last_move == ID::from("fusionflare") {
            // this.debug('double power');
            battle.debug("double power");

            // return this.chainModify(2);
            return EventResult::Number(battle.chain_modify(2.0 as f32));
        }
    }

    EventResult::Continue
}


//! Psyblade Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, source) {
///     if (this.field.isTerrain('electricterrain')) {
///         this.debug('psyblade electric terrain boost');
///         return this.chainModify(1.5);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, _base_power: i32, _pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    // if (this.field.isTerrain('electricterrain')) {
    let is_electric_terrain = battle.field.terrain == ID::from("electricterrain");

    if is_electric_terrain {
        // this.debug('psyblade electric terrain boost');
        battle.debug("psyblade electric terrain boost");

        // return this.chainModify(1.5);
        return EventResult::Number(battle.chain_modify_fraction(3, 2)); // 1.5 = 3/2
    }

    EventResult::Continue
}


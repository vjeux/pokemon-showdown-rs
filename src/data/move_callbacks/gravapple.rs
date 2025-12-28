//! Grav Apple Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower) {
///     if (this.field.getPseudoWeather('gravity')) {
///         return this.chainModify(1.5);
///     }
/// }
pub fn on_base_power(
    battle: &mut Battle,
    _base_power: i32,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    // if (this.field.getPseudoWeather('gravity')) {
    let has_gravity = battle
        .field
        .pseudo_weather
        .contains_key(&ID::from("gravity"));

    if has_gravity {
        // return this.chainModify(1.5);
        return EventResult::Number(battle.chain_modify(1.5_f32));
    }

    EventResult::Continue
}

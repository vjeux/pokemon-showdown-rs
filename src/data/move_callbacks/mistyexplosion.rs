//! Misty Explosion Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, source) {
///     if (this.field.isTerrain('mistyterrain') && source.isGrounded()) {
///         this.debug('misty terrain boost');
///         return this.chainModify(1.5);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, _base_power: i32, pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let source = pokemon_pos;

    // if (this.field.isTerrain('mistyterrain') && source.isGrounded()) {
    let is_misty_terrain = battle.field.terrain == ID::from("mistyterrain");

    if !is_misty_terrain {
        return EventResult::Continue;
    }

    let is_grounded = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.is_grounded()
    };

    if is_grounded {
        // this.debug('misty terrain boost');
        // (debug is typically not needed in Rust implementation)

        // return this.chainModify(1.5);
        return EventResult::Number(battle.chain_modify_fraction(3, 2)); // 1.5 = 3/2
    }

    EventResult::Continue
}


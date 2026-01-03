//! Grass Pelt Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyDef(pokemon) {
///     if (this.field.isTerrain('grassyterrain')) return this.chainModify(1.5);
/// }
pub fn on_modify_def(battle: &mut Battle, _def: i32, _defender_pos: (usize, usize), _attacker_pos: (usize, usize), _move_id: &str) -> EventResult {
    if battle.field.has_terrain(&crate::ID::from("grassyterrain")) {
        return EventResult::Number(battle.chain_modify(1.5));
    }
    EventResult::Continue
}


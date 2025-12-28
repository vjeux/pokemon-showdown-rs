//! Nature Power Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, pokemon) {
///     let move = 'triattack';
///     if (this.field.isTerrain('electricterrain')) {
///         move = 'thunderbolt';
///     } else if (this.field.isTerrain('grassyterrain')) {
///         move = 'energyball';
///     } else if (this.field.isTerrain('mistyterrain')) {
///         move = 'moonblast';
///     } else if (this.field.isTerrain('psychicterrain')) {
///         move = 'psychic';
///     }
///     this.actions.useMove(move, pokemon, { target });
///     return null;
/// }
pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;
    use crate::battle_actions;

    let pokemon = source_pos;
    let target = target_pos;

    // let move = 'triattack';
    let mut move_id = "triattack";

    // if (this.field.isTerrain('electricterrain')) {
    if battle.field.terrain == ID::from("electricterrain") {
        // move = 'thunderbolt';
        move_id = "thunderbolt";
    // } else if (this.field.isTerrain('grassyterrain')) {
    } else if battle.field.terrain == ID::from("grassyterrain") {
        // move = 'energyball';
        move_id = "energyball";
    // } else if (this.field.isTerrain('mistyterrain')) {
    } else if battle.field.terrain == ID::from("mistyterrain") {
        // move = 'moonblast';
        move_id = "moonblast";
    // } else if (this.field.isTerrain('psychicterrain')) {
    } else if battle.field.terrain == ID::from("psychicterrain") {
        // move = 'psychic';
        move_id = "psychic";
    }

    // this.actions.useMove(move, pokemon, { target });
    battle_actions::use_move(battle, move_id, pokemon, Some(target), None, None, None);

    // return null;
    EventResult::Stop
}


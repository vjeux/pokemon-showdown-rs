//! Expanding Force Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, source) {
///     if (this.field.isTerrain('psychicterrain') && source.isGrounded()) {
///         this.debug('terrain buff');
///         return this.chainModify(1.5);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, base_power: i32, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let source = pokemon_pos;

    // if (this.field.isTerrain('psychicterrain') && source.isGrounded()) {
    let is_psychic_terrain = battle.field.is_terrain(&ID::from("psychicterrain"));

    if is_psychic_terrain {
        let is_grounded = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.is_grounded(battle)
        };

        if is_grounded {
            // this.debug('terrain buff');
            battle.debug("terrain buff");

            // return this.chainModify(1.5);
            return EventResult::ChainModify(1.5);
        }
    }

    EventResult::Continue
}

/// onModifyMove(move, source, target) {
///     if (this.field.isTerrain('psychicterrain') && source.isGrounded()) {
///         move.target = 'allAdjacentFoes';
///     }
/// }
pub fn on_modify_move(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let source = pokemon_pos;

    // if (this.field.isTerrain('psychicterrain') && source.isGrounded()) {
    //     move.target = 'allAdjacentFoes';
    // }
    let is_psychic_terrain = battle.field.is_terrain(&ID::from("psychicterrain"));

    if is_psychic_terrain {
        let is_grounded = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.is_grounded(battle)
        };

        if is_grounded {
            // move.target = 'allAdjacentFoes';
            // We need to modify the current move's target
            if let Some(ref move_id) = battle.current_move {
                if let Some(move_data) = battle.dex.get_move_by_id_mut(move_id) {
                    move_data.target = Some("allAdjacentFoes".to_string());
                }
            }
        }
    }

    EventResult::Continue
}


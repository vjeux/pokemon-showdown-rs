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
    let is_psychic_terrain = battle.field.is_terrain("psychicterrain");

    if is_psychic_terrain {
        let is_grounded = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.is_grounded()
        };

        if is_grounded {
            // this.debug('terrain buff');
            battle.debug("terrain buff");

            // return this.chainModify(1.5);
            return EventResult::Number(battle.chain_modify(1.5 as f32));
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
    let is_psychic_terrain = battle.field.is_terrain("psychicterrain");

    if is_psychic_terrain {
        let is_grounded = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.is_grounded()
        };

        if is_grounded {
            // move.target = 'allAdjacentFoes';
            // We need to modify the current move's target
            if let Some(ref mut active_move) = battle.active_move {
                active_move.target = "allAdjacentFoes".to_string();
            }
        }
    }

    EventResult::Continue
}


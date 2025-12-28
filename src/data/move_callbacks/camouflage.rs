//! Camouflage Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::event::EventResult;
use crate::dex_data::ID;

/// onHit(target) {
///     let newType = 'Normal';
///     if (this.field.isTerrain('electricterrain')) {
///         newType = 'Electric';
///     } else if (this.field.isTerrain('grassyterrain')) {
///         newType = 'Grass';
///     } else if (this.field.isTerrain('mistyterrain')) {
///         newType = 'Fairy';
///     } else if (this.field.isTerrain('psychicterrain')) {
///         newType = 'Psychic';
///     }
///
///     if (target.getTypes().join() === newType || !target.setType(newType)) return false;
///     this.add('-start', target, 'typechange', newType);
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // Get the target
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // let newType = 'Normal';
    let mut new_type = ID::from("Normal");

    // if (this.field.isTerrain('electricterrain')) {
    //     newType = 'Electric';
    // } else if (this.field.isTerrain('grassyterrain')) {
    //     newType = 'Grass';
    // } else if (this.field.isTerrain('mistyterrain')) {
    //     newType = 'Fairy';
    // } else if (this.field.isTerrain('psychicterrain')) {
    //     newType = 'Psychic';
    // }
    if battle.field.is_terrain(&ID::from("electricterrain")) {
        new_type = ID::from("Electric");
    } else if battle.field.is_terrain(&ID::from("grassyterrain")) {
        new_type = ID::from("Grass");
    } else if battle.field.is_terrain(&ID::from("mistyterrain")) {
        new_type = ID::from("Fairy");
    } else if battle.field.is_terrain(&ID::from("psychicterrain")) {
        new_type = ID::from("Psychic");
    }

    // if (target.getTypes().join() === newType || !target.setType(newType)) return false;
    let target_types = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.get_types(false)
    };

    // Check if types match (single type case)
    let types_match = target_types.len() == 1 && target_types[0] == new_type;

    if types_match {
        return EventResult::Boolean(false);
    }

    // target.setType(newType)
    let set_type_success = {
        let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.set_type(vec![new_type.clone()])
    };

    if !set_type_success {
        return EventResult::Boolean(false);
    }

    // this.add('-start', target, 'typechange', newType);
    let target_arg = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        Arg::from(target_pokemon)
    };

    battle.add("-start", &[target_arg, "typechange".into(), new_type.to_string().into()]);

    EventResult::Continue
}

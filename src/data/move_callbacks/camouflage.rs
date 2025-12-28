//! Camouflage Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// ```ignore
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
/// ```
pub fn on_hit(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    // Get the target
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // let newType = 'Normal';
    let mut new_type = String::from("Normal");

    // if (this.field.isTerrain('electricterrain')) {
    //     newType = 'Electric';
    // } else if (this.field.isTerrain('grassyterrain')) {
    //     newType = 'Grass';
    // } else if (this.field.isTerrain('mistyterrain')) {
    //     newType = 'Fairy';
    // } else if (this.field.isTerrain('psychicterrain')) {
    //     newType = 'Psychic';
    // }
    if battle.field.is_terrain("electricterrain") {
        new_type = String::from("Electric");
    } else if battle.field.is_terrain("grassyterrain") {
        new_type = String::from("Grass");
    } else if battle.field.is_terrain("mistyterrain") {
        new_type = String::from("Fairy");
    } else if battle.field.is_terrain("psychicterrain") {
        new_type = String::from("Psychic");
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
    {
        let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.set_type(vec![new_type.clone()]);
    }

    // this.add('-start', target, 'typechange', newType);
    let target_ident = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.get_slot()
    };

    battle.add(
        "-start",
        &[
            target_ident.as_str().into(),
            "typechange".into(),
            new_type.into(),
        ],
    );

    EventResult::Continue
}

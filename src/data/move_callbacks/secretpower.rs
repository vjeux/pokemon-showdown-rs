//! Secret Power Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyMove(move, pokemon) {
///     if (this.field.isTerrain('')) return;
///     move.secondaries = [];
///     if (this.field.isTerrain('electricterrain')) {
///         move.secondaries.push({
///             chance: 30,
///             status: 'par',
///         });
///     } else if (this.field.isTerrain('grassyterrain')) {
///         move.secondaries.push({
///             chance: 30,
///             status: 'slp',
///         });
///     } else if (this.field.isTerrain('mistyterrain')) {
///         move.secondaries.push({
///             chance: 30,
///             boosts: {
///                 spa: -1,
///             },
///         });
///     } else if (this.field.isTerrain('psychicterrain')) {
///         move.secondaries.push({
///             chance: 30,
///             boosts: {
///                 spe: -1,
///             },
///         });
///     }
/// }
pub fn on_modify_move(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    // if (this.field.isTerrain('')) return;
    if battle.field.terrain == ID::from("") {
        return EventResult::Continue;
    }

    let terrain_id = &battle.field.terrain;

    // move.secondaries = [];
    let active_move = match &battle.active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    active_move.borrow_mut().secondaries = vec![];

    // if (this.field.isTerrain('electricterrain')) {
    if *terrain_id == ID::from("electricterrain") {
        // move.secondaries.push({
        //     chance: 30,
        //     status: 'par',
        // });
        let secondary = crate::dex::MoveSecondary {
            chance: Some(30),
            status: Some("par".to_string()),
            ..Default::default()
        };
        active_move.borrow_mut().secondaries.push(secondary);
    } else if *terrain_id == ID::from("grassyterrain") {
        // move.secondaries.push({
        //     chance: 30,
        //     status: 'slp',
        // });
        let secondary = crate::dex::MoveSecondary {
            chance: Some(30),
            status: Some("slp".to_string()),
            ..Default::default()
        };
        active_move.borrow_mut().secondaries.push(secondary);
    } else if *terrain_id == ID::from("mistyterrain") {
        // move.secondaries.push({
        //     chance: 30,
        //     boosts: {
        //         spa: -1,
        //     },
        // });
        let mut boosts = crate::dex_data::BoostsTable::new();
        boosts.spa = -1;
        let secondary = crate::dex::MoveSecondary {
            chance: Some(30),
            boosts: Some(boosts),
            ..Default::default()
        };
        active_move.borrow_mut().secondaries.push(secondary);
    } else if *terrain_id == ID::from("psychicterrain") {
        // move.secondaries.push({
        //     chance: 30,
        //     boosts: {
        //         spe: -1,
        //     },
        // });
        let mut boosts = crate::dex_data::BoostsTable::new();
        boosts.spe = -1;
        let secondary = crate::dex::MoveSecondary {
            chance: Some(30),
            boosts: Some(boosts),
            ..Default::default()
        };
        active_move.borrow_mut().secondaries.push(secondary);
    }

    EventResult::Continue
}

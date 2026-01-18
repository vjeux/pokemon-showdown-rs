//! Ivy Cudgel Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onPrepareHit(target, source, move) {
///     if (move.type !== "Grass") {
///         this.attrLastMove('[anim] Ivy Cudgel ' + move.type);
///     }
/// }
pub fn on_prepare_hit(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // if (move.type !== "Grass") {
    //     this.attrLastMove('[anim] Ivy Cudgel ' + move.type);
    // }
    let move_type = battle.active_move.as_ref().map(|m| m.move_type.clone());

    if let Some(move_type) = move_type {
        if move_type != "Grass" {
            let anim_str = format!("[anim] Ivy Cudgel {}", move_type.as_str());
            battle.attr_last_move(&[&anim_str]);
        }
    }

    EventResult::Continue
}

/// onModifyType(move, pokemon) {
///     switch (pokemon.species_id.as_str()) {
///     case 'Ogerpon-Wellspring': case 'Ogerpon-Wellspring-Tera':
///         move.type = 'Water';
///         break;
///     case 'Ogerpon-Hearthflame': case 'Ogerpon-Hearthflame-Tera':
///         move.type = 'Fire';
///         break;
///     case 'Ogerpon-Cornerstone': case 'Ogerpon-Cornerstone-Tera':
///         move.type = 'Rock';
///         break;
///     }
/// }
pub fn on_modify_type(
    battle: &mut Battle,
    _active_move: Option<&crate::battle_actions::ActiveMove>,
    pokemon_pos: (usize, usize),
) -> EventResult {
    let pokemon = pokemon_pos;

    // switch (pokemon.species_id.as_str()) {
    let species_name = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => {
                return EventResult::Continue;
            }
        };
        pokemon_pokemon.species_id.as_str()
    };

    // case 'Ogerpon-Wellspring': case 'Ogerpon-Wellspring-Tera':
    //     move.type = 'Water';
    //     break;
    // case 'Ogerpon-Hearthflame': case 'Ogerpon-Hearthflame-Tera':
    //     move.type = 'Fire';
    //     break;
    // case 'Ogerpon-Cornerstone': case 'Ogerpon-Cornerstone-Tera':
    //     move.type = 'Rock';
    //     break;
    let new_type = match species_name {
        "ogerponwellspring" | "ogerponwellspringtera" => Some("Water"),
        "ogerponhearthflame" | "ogerponhearthflametera" => Some("Fire"),
        "ogerponcornerstone" | "ogerponcornerstonetera" => Some("Rock"),
        _ => None,
    };

    if let Some(new_type_str) = new_type {
        if let Some(ref mut active_move) = battle.active_move {
            active_move.move_type = new_type_str.to_string();
        }
    }

    EventResult::Continue
}

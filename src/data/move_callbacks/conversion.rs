//! Conversion Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onHit(target) {
///     const type = this.dex.moves.get(target.moveSlots[0].id).type;
///     if (target.hasType(type) || !target.setType(type)) return false;
///     this.add('-start', target, 'typechange', type);
/// }
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

    // const type = this.dex.moves.get(target.moveSlots[0].id).type;
    let move_type = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        if target_pokemon.move_slots.is_empty() {
            return EventResult::Boolean(false);
        }

        let first_move_id = &target_pokemon.move_slots[0].id;
        let move_data = match battle.dex.moves().get_by_id(first_move_id) {
            Some(m) => m,
            None => return EventResult::Continue,
        };

        move_data.move_type.clone()
    };

    // if (target.hasType(type) || !target.setType(type)) return false;
    let has_type = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.has_type(battle, &move_type)
    };

    // JavaScript: || short-circuits, so setType is only called if hasType is false
    // If target already has the type, fail without calling setType
    if has_type {
        return EventResult::Boolean(false);
    }

    // Try to set the type and check if it succeeded
    let set_type_succeeded = Pokemon::set_type(battle, target, vec![move_type.clone()], false);
    if !set_type_succeeded {
        return EventResult::Boolean(false);
    }

    // this.add('-start', target, 'typechange', type);
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
            move_type.to_string().into(),
        ],
    );

    EventResult::Continue
}

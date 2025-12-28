//! Conversion Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::event::EventResult;
use crate::dex_data::ID;

/// onHit(target) {
///     const type = this.dex.moves.get(target.moveSlots[0].id).type;
///     if (target.hasType(type) || !target.setType(type)) return false;
///     this.add('-start', target, 'typechange', type);
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
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
        let move_data = match battle.dex.get_move_by_id(first_move_id) {
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
        target_pokemon.has_type(&move_type)
    };

    if has_type {
        return EventResult::Boolean(false);
    }

    {
        let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.set_type(vec![move_type.clone()]);
    }

    // this.add('-start', target, 'typechange', type);
    let target_ident = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.get_slot()
    };

    battle.add("-start", &[target_ident.as_str().into(), "typechange".into(), move_type.to_string().into()]);

    EventResult::Continue
}

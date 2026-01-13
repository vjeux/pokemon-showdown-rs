//! Mimic Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// ```ignore
/// onHit(target, source) {
///     const move = target.lastMove;
///     if (source.transformed || !move || move.flags['failmimic'] || source.moves.includes(move.id)) {
///         return false;
///     }
///     if (move.isZ || move.isMax) return false;
///     const mimicIndex = source.moves.indexOf('mimic');
///     if (mimicIndex < 0) return false;
///
///     source.moveSlots[mimicIndex] = {
///         move: move.name,
///         id: move.id,
///         pp: move.pp,
///         maxpp: move.pp,
///         target: move.target,
///         disabled: false,
///         used: false,
///         virtual: true,
///     };
///     this.add('-start', source, 'Mimic', move.name);
/// }
/// ```
pub fn on_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),  // First param is target
    source_pos: Option<(usize, usize)>,  // Second param is source
) -> EventResult {
    use crate::dex_data::ID;

    let target = target_pos;
    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Boolean(false),
    };

    // const move = target.lastMove;
    let last_move_id = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Boolean(false),
        };
        target_pokemon.last_move.clone()
    };

    // if (source.transformed || !move || move.flags['failmimic'] || source.moves.includes(move.id)) {
    //     return false;
    // }
    let move_id = match last_move_id {
        Some(id) => id,
        None => return EventResult::Boolean(false),
    };

    let move_data = match battle.dex.moves().get_by_id(&move_id) {
        Some(m) => m,
        None => return EventResult::Boolean(false),
    };

    // Clone all needed data from move_data to avoid holding the borrow
    let move_name = move_data.name.clone();
    let move_id_clone = move_data.id.clone();
    let move_pp = move_data.pp;
    let move_target = move_data.target.clone();
    let move_flags_has_failmimic = move_data.flags.contains_key("failmimic");
    let move_is_z = move_data.is_z.is_some();
    let move_is_max = move_data.is_max.is_some();

    let (source_transformed, source_has_move) = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Boolean(false),
        };
        let transformed = source_pokemon.transformed;
        let has_move = source_pokemon
            .move_slots
            .iter()
            .any(|slot| slot.id == move_id);
        (transformed, has_move)
    };

    if source_transformed || move_flags_has_failmimic || source_has_move {
        return EventResult::Boolean(false);
    }

    // if (move.isZ || move.isMax) return false;
    if move_is_z || move_is_max {
        return EventResult::Boolean(false);
    }

    // const mimicIndex = source.moves.indexOf('mimic');
    // if (mimicIndex < 0) return false;
    let mimic_index = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Boolean(false),
        };
        source_pokemon
            .move_slots
            .iter()
            .position(|slot| slot.id == ID::from("mimic"))
    };

    let mimic_index = match mimic_index {
        Some(idx) => idx,
        None => return EventResult::Boolean(false),
    };

    // source.moveSlots[mimicIndex] = {
    //     move: move.name,
    //     id: move.id,
    //     pp: move.pp,
    //     maxpp: move.pp,
    //     target: move.target,
    //     disabled: false,
    //     used: false,
    //     virtual: true,
    // };
    {
        let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Boolean(false),
        };

        if let Some(slot) = source_pokemon.move_slots.get_mut(mimic_index) {
            slot.move_name = move_name.clone();
            slot.id = move_id_clone.clone();
            slot.pp = move_pp as u8;
            slot.maxpp = move_pp as u8;
            slot.target = Some(move_target.clone());
            slot.disabled = false;
            slot.used = false;
            slot.virtual_move = true;
        }
    }

    // this.add('-start', source, 'Mimic', move.name);
    let source_arg = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Boolean(false),
        };
        source_pokemon.get_slot()
    };

    battle.add(
        "-start",
        &[source_arg.into(), "Mimic".into(), move_name.into()],
    );

    EventResult::Continue
}

//! Sketch Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source) {
///     const move = target.lastMove;
///     if (source.transformed || !move || source.moves.includes(move.id)) return false;
///     if (move.flags['nosketch'] || move.isZ || move.isMax) return false;
///     const sketchIndex = source.moves.indexOf('sketch');
///     if (sketchIndex < 0) return false;
///     const sketchedMove = {
///         move: move.name,
///         id: move.id,
///         pp: move.pp,
///         maxpp: move.pp,
///         target: move.target,
///         disabled: false,
///         used: false,
///     };
///     source.moveSlots[sketchIndex] = sketchedMove;
///     source.baseMoveSlots[sketchIndex] = sketchedMove;
///     this.add('-activate', source, 'move: Sketch', move.name);
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    // onHit(target, source) {
    //     const move = target.lastMove;
    //     if (source.transformed || !move || source.moves.includes(move.id)) return false;
    //     if (move.flags['nosketch'] || move.isZ || move.isMax) return false;
    //     const sketchIndex = source.moves.indexOf('sketch');
    //     if (sketchIndex < 0) return false;
    //     const sketchedMove = {
    //         move: move.name,
    //         id: move.id,
    //         pp: move.pp,
    //         maxpp: move.pp,
    //         target: move.target,
    //         disabled: false,
    //         used: false,
    //     };
    //     source.moveSlots[sketchIndex] = sketchedMove;
    //     source.baseMoveSlots[sketchIndex] = sketchedMove;
    //     this.add('-activate', source, 'move: Sketch', move.name);
    // }
    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const move = target.lastMove;
    let move_id = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        match &target_pokemon.last_move {
            Some(m) => m.clone(),
            None => return EventResult::Boolean(false),
        }
    };

    // if (source.transformed || !move || source.moves.includes(move.id)) return false;
    let (is_transformed, has_move) = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let is_transformed = source_pokemon.transformed;
        let has_move = source_pokemon.move_slots.iter().any(|slot| slot.id == move_id);
        (is_transformed, has_move)
    };

    if is_transformed || has_move {
        return EventResult::Boolean(false);
    }

    // if (move.flags['nosketch'] || move.isZ || move.isMax) return false;
    let move_data = match battle.dex.get_move_by_id(&move_id) {
        Some(m) => m,
        None => return EventResult::Boolean(false),
    };

    let has_nosketch = move_data.flags.get("nosketch").copied().unwrap_or(0) != 0;
    if has_nosketch || move_data.is_z || move_data.is_max {
        return EventResult::Boolean(false);
    }

    // const sketchIndex = source.moves.indexOf('sketch');
    // if (sketchIndex < 0) return false;
    let sketch_index = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.move_slots.iter().position(|slot| slot.id == ID::from("sketch"))
    };

    let sketch_index = match sketch_index {
        Some(idx) => idx,
        None => return EventResult::Boolean(false),
    };

    // const sketchedMove = {
    //     move: move.name,
    //     id: move.id,
    //     pp: move.pp,
    //     maxpp: move.pp,
    //     target: move.target,
    //     disabled: false,
    //     used: false,
    // };
    // source.moveSlots[sketchIndex] = sketchedMove;
    // source.baseMoveSlots[sketchIndex] = sketchedMove;
    battle.set_move_slot(source, sketch_index, &move_id, move_data.pp, move_data.pp);

    // this.add('-activate', source, 'move: Sketch', move.name);
    let source_arg = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        crate::battle::Arg::from(source_pokemon)
    };

    battle.add("-activate", &[
        source_arg,
        "move: Sketch".into(),
        move_data.name.clone().into(),
    ]);

    EventResult::Continue
}


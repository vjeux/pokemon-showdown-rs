//! Sleep Talk Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTry(source) {
///     return source.status === 'slp' || source.hasAbility('comatose');
/// }
pub fn on_try(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    // onTry(source) {
    //     return source.status === 'slp' || source.hasAbility('comatose');
    // }
    let source = source_pos;

    // return source.status === 'slp' || source.hasAbility('comatose');
    let has_sleep_or_comatose = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let has_sleep = source_pokemon.status == ID::from("slp");
        let has_comatose = source_pokemon.has_ability(battle, &["comatose"]);

        has_sleep || has_comatose
    };

    EventResult::Boolean(has_sleep_or_comatose)
}

/// onHit(pokemon) {
///     const moves = [];
///     for (const moveSlot of pokemon.moveSlots) {
///         const moveid = moveSlot.id;
///         if (!moveid) continue;
///         const move = this.dex.moves.get(moveid);
///         if (move.flags['nosleeptalk'] || move.flags['charge'] || (move.isZ && move.basePower !== 1) || move.isMax) {
///             continue;
///         }
///         moves.push(moveid);
///     }
///     let randomMove = '';
///     if (moves.length) randomMove = this.sample(moves);
///     if (!randomMove) {
///         return false;
///     }
///     this.actions.useMove(randomMove, pokemon);
/// }
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // onHit(pokemon) {
    //     const moves = [];
    //     for (const moveSlot of pokemon.moveSlots) {
    //         const moveid = moveSlot.id;
    //         if (!moveid) continue;
    //         const move = this.dex.moves.get(moveid);
    //         if (move.flags['nosleeptalk'] || move.flags['charge'] || (move.isZ && move.basePower !== 1) || move.isMax) {
    //             continue;
    //         }
    //         moves.push(moveid);
    //     }
    //     let randomMove = '';
    //     if (moves.length) randomMove = this.sample(moves);
    //     if (!randomMove) {
    //         return false;
    //     }
    //     this.actions.useMove(randomMove, pokemon);
    // }
    let pokemon = pokemon_pos;

    // const moves = [];
    let mut moves = Vec::new();

    // for (const moveSlot of pokemon.moveSlots) {
    let move_slots = {
        let pokemon_data = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_data.move_slots.clone()
    };

    for move_slot in &move_slots {
        // const moveid = moveSlot.id;
        let moveid = &move_slot.id;

        // if (!moveid) continue;
        if moveid.is_empty() {
            continue;
        }

        // const move = this.dex.moves.get(moveid);
        let move_data = battle.dex.moves().get_by_id(moveid);
        let move_data = match move_data {
            Some(m) => m,
            None => continue,
        };

        // if (move.flags['nosleeptalk'] || move.flags['charge'] || (move.isZ && move.basePower !== 1) || move.isMax) {
        //     continue;
        // }
        if move_data.flags.get("nosleeptalk").copied().unwrap_or(0) != 0
            || move_data.flags.get("charge").copied().unwrap_or(0) != 0
            || (move_data.is_z.is_some() && move_data.base_power != 1)
            || move_data.is_max.is_some()
        {
            continue;
        }

        // moves.push(moveid);
        moves.push(moveid.clone());
    }

    // let randomMove = '';
    // if (moves.length) randomMove = this.sample(moves);
    let random_move = if !moves.is_empty() {
        battle.sample(&moves)
    } else {
        None
    };

    // if (!randomMove) {
    //     return false;
    // }
    let _random_move = match random_move {
        Some(m) => m,
        None => return EventResult::Boolean(false),
    };

    // this.actions.useMove(randomMove, pokemon);
    battle.use_move(&_random_move, pokemon, None, None, None, None);

    EventResult::Continue
}

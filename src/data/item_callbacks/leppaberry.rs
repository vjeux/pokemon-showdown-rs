//! Leppa Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;
use crate::Pokemon;

/// onUpdate(pokemon) {
///     if (!pokemon.hp) return;
///     if (pokemon.moveSlots.some(move => move.pp === 0)) {
///         pokemon.eatItem();
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (!pokemon.hp) return;
    let has_empty_pp_move = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        if pokemon.hp == 0 {
            return EventResult::Continue;
        }

        // if (pokemon.moveSlots.some(move => move.pp === 0))
        pokemon.move_slots.iter().any(|move_slot| move_slot.pp == 0)
    };

    if has_empty_pp_move {
        // pokemon.eatItem();
        let _pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        Pokemon::eat_item(battle, pokemon_pos, false, None, None);
    }

    EventResult::Continue
}

/// onEat(pokemon) {
///     const moveSlot = pokemon.moveSlots.find(move => move.pp === 0) ||
///         pokemon.moveSlots.find(move => move.pp < move.maxpp);
///     if (!moveSlot) return;
///     moveSlot.pp += 10;
///     if (moveSlot.pp > moveSlot.maxpp) moveSlot.pp = moveSlot.maxpp;
///     this.add('-activate', pokemon, 'item: Leppa Berry', moveSlot.move, '[consumed]');
/// }
pub fn on_eat(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // const moveSlot = pokemon.moveSlots.find(move => move.pp === 0) ||
    //     pokemon.moveSlots.find(move => move.pp < move.maxpp);
    let (move_slot_index, move_name, pokemon_ident) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        // Find first move with pp === 0
        let mut move_slot_idx = pokemon.move_slots.iter()
            .position(|move_slot| move_slot.pp == 0);

        // If no move with pp === 0, find first move with pp < maxpp
        if move_slot_idx.is_none() {
            move_slot_idx = pokemon.move_slots.iter()
                .position(|move_slot| move_slot.pp < move_slot.maxpp);
        }

        // if (!moveSlot) return;
        match move_slot_idx {
            Some(idx) => {
                let move_name = pokemon.move_slots[idx].move_name.clone();
                let ident = pokemon.get_slot();
                (idx, move_name, ident)
            },
            None => return EventResult::Continue,
        }
    };

    // moveSlot.pp += 10;
    // if (moveSlot.pp > moveSlot.maxpp) moveSlot.pp = moveSlot.maxpp;
    {
        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let move_slot = &mut pokemon_mut.move_slots[move_slot_index];
        move_slot.pp = move_slot.pp.saturating_add(10).min(move_slot.maxpp);
    }

    // this.add('-activate', pokemon, 'item: Leppa Berry', moveSlot.move, '[consumed]');
    battle.add("-activate", &[
        pokemon_ident.as_str().into(),
        "item: Leppa Berry".into(),
        move_name.into(),
        "[consumed]".into()
    ]);

    EventResult::Continue
}

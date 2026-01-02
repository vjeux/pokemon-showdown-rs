//! Mystery Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onUpdate(pokemon) {
///     if (!pokemon.hp) return;
///     const moveSlot = pokemon.lastMove && pokemon.getMoveData(pokemon.lastMove.id);
///     if (moveSlot && moveSlot.pp === 0) {
///         pokemon.addVolatile('leppaberry');
///         pokemon.volatiles['leppaberry'].moveSlot = moveSlot;
///         pokemon.eatItem();
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (!pokemon.hp) return;
    let (should_eat, move_slot_idx) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        if pokemon.hp == 0 {
            return EventResult::Continue;
        }

        // const moveSlot = pokemon.lastMove && pokemon.getMoveData(pokemon.lastMove.id);
        // if (moveSlot && moveSlot.pp === 0)
        if let Some(ref last_move) = pokemon.last_move_used {
            let move_slot_idx = pokemon.move_slots.iter()
                .position(|m| &m.id == last_move);

            if let Some(idx) = move_slot_idx {
                let has_zero_pp = pokemon.move_slots[idx].pp == 0;
                (has_zero_pp, Some(idx))
            } else {
                (false, None)
            }
        } else {
            (false, None)
        }
    };

    if should_eat {
        use crate::dex_data::ID;

        // pokemon.addVolatile('leppaberry');
        // pokemon.volatiles['leppaberry'].moveSlot = moveSlot;
        Pokemon::add_volatile(battle, pokemon_pos, ID::from("leppaberry"), None, None, None);

        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        if let Some(volatile) = pokemon_mut.get_volatile_mut(&ID::from("leppaberry")) {
            // Store the move slot index in the volatile's data field
            volatile.data.insert(
                "move_slot_index".to_string(),
                serde_json::json!(move_slot_idx.unwrap_or(0))
            );
        }

        // pokemon.eatItem();
        pokemon_mut.eat_item(false, None, None);
    }

    EventResult::Continue
}

/// onEat(pokemon) {
///     let moveSlot;
///     if (pokemon.volatiles['leppaberry']) {
///         moveSlot = pokemon.volatiles['leppaberry'].moveSlot;
///         pokemon.removeVolatile('leppaberry');
///     } else {
///         let pp = 99;
///         for (const possibleMoveSlot of pokemon.moveSlots) {
///             if (possibleMoveSlot.pp < pp) {
///                 moveSlot = possibleMoveSlot;
///                 pp = moveSlot.pp;
///             }
///         }
///     }
///     moveSlot.pp += 5;
///     if (moveSlot.pp > moveSlot.maxpp) moveSlot.pp = moveSlot.maxpp;
///     this.add('-activate', pokemon, 'item: Mystery Berry', moveSlot.move);
/// }
pub fn on_eat(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    // let moveSlot;
    // if (pokemon.volatiles['leppaberry']) {
    //     moveSlot = pokemon.volatiles['leppaberry'].moveSlot;
    //     pokemon.removeVolatile('leppaberry');
    // }
    let (move_slot_index, move_name, pokemon_ident) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let move_slot_idx = if let Some(volatile) = pokemon.get_volatile(&ID::from("leppaberry")) {
            // Retrieve move slot index from volatile data
            volatile.data.get("move_slot_index")
                .and_then(|v| v.as_u64())
                .map(|v| v as usize)
        } else {
            // Find move with lowest PP
            let mut min_pp = 99;
            let mut min_idx = None;
            for (idx, move_slot) in pokemon.move_slots.iter().enumerate() {
                if move_slot.pp < min_pp {
                    min_pp = move_slot.pp;
                    min_idx = Some(idx);
                }
            }
            min_idx
        };

        match move_slot_idx {
            Some(idx) if idx < pokemon.move_slots.len() => {
                let move_name = pokemon.move_slots[idx].move_name.clone();
                let ident = pokemon.get_slot();
                (idx, move_name, ident)
            },
            _ => return EventResult::Continue,
        }
    };

    // Remove volatile if present
    {
        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_mut.remove_volatile(&ID::from("leppaberry"));
    }

    // moveSlot.pp += 5;
    // if (moveSlot.pp > moveSlot.maxpp) moveSlot.pp = moveSlot.maxpp;
    {
        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        if move_slot_index < pokemon_mut.move_slots.len() {
            let move_slot = &mut pokemon_mut.move_slots[move_slot_index];
            move_slot.pp = move_slot.pp.saturating_add(5).min(move_slot.maxpp);
        }
    }

    // this.add('-activate', pokemon, 'item: Mystery Berry', moveSlot.move);
    battle.add("-activate", &[
        pokemon_ident.as_str().into(),
        "item: Mystery Berry".into(),
        move_name.into()
    ]);

    EventResult::Continue
}

//! Mystery Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

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
        // Note: JS uses pokemon.lastMove (set in moveUsed only when !externalMove),
        // NOT pokemon.lastMoveUsed (set unconditionally in useMoveInner).
        // This matters for moves like Mirror Move where the copied move shouldn't
        // override lastMove.
        if let Some(ref last_move_id) = pokemon.last_move {
            let move_slot_idx = pokemon.move_slots.iter()
                .position(|m| m.id == *last_move_id);

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
        Pokemon::add_volatile(battle, pokemon_pos, ID::from("leppaberry"), None, None, None, None);

        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        if let Some(volatile) = pokemon_mut.get_volatile_mut(&ID::from("leppaberry")) {
            // Store the move slot index in the volatile's move_slot_index field
            volatile.move_slot_index = move_slot_idx;
        }

        // pokemon.eatItem();
        Pokemon::eat_item(battle, pokemon_pos, false, None, None);
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
    use crate::debug_elog;

    debug_elog!("[MYSTERYBERRY] on_eat called for pokemon at {:?}", pokemon_pos);

    // let moveSlot;
    // if (pokemon.volatiles['leppaberry']) {
    //     moveSlot = pokemon.volatiles['leppaberry'].moveSlot;
    //     pokemon.removeVolatile('leppaberry');
    // }
    let (move_slot_index, move_name, pokemon_ident) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => {
                debug_elog!("[MYSTERYBERRY] pokemon_at returned None for {:?}", pokemon_pos);
                return EventResult::Continue;
            }
        };

        debug_elog!("[MYSTERYBERRY] Found pokemon: {} with {} move slots", pokemon.name, pokemon.move_slots.len());

        let has_leppaberry = pokemon.get_volatile(&ID::from("leppaberry")).is_some();
        debug_elog!("[MYSTERYBERRY] Has leppaberry volatile: {}", has_leppaberry);

        let move_slot_idx = if let Some(volatile) = pokemon.get_volatile(&ID::from("leppaberry")) {
            // Retrieve move slot index from volatile move_slot_index field
            debug_elog!("[MYSTERYBERRY] leppaberry volatile.move_slot_index = {:?}", volatile.move_slot_index);
            volatile.move_slot_index
        } else {
            // Find move with lowest PP
            let mut min_pp = 99;
            let mut min_idx = None;
            for (idx, move_slot) in pokemon.move_slots.iter().enumerate() {
                debug_elog!("[MYSTERYBERRY] move_slot[{}]: {} pp={}/{}", idx, move_slot.id, move_slot.pp, move_slot.maxpp);
                if move_slot.pp < min_pp {
                    min_pp = move_slot.pp;
                    min_idx = Some(idx);
                }
            }
            debug_elog!("[MYSTERYBERRY] Found min_idx={:?} with pp={}", min_idx, min_pp);
            min_idx
        };

        debug_elog!("[MYSTERYBERRY] move_slot_idx = {:?}", move_slot_idx);

        match move_slot_idx {
            Some(idx) if idx < pokemon.move_slots.len() => {
                let move_name = pokemon.move_slots[idx].move_name.clone();
                let ident = pokemon.get_slot();
                debug_elog!("[MYSTERYBERRY] Will restore PP for move {} at index {}", move_name, idx);
                (idx, move_name, ident)
            },
            _ => {
                debug_elog!("[MYSTERYBERRY] Exiting early: move_slot_idx={:?} is invalid", move_slot_idx);
                return EventResult::Continue;
            }
        }
    };

    // Remove volatile if present
    {
        debug_elog!("[MYSTERYBERRY] Removing leppaberry volatile");
        Pokemon::remove_volatile(battle, pokemon_pos, &ID::from("leppaberry"));
    }

    // moveSlot.pp += 5;
    // if (moveSlot.pp > moveSlot.maxpp) moveSlot.pp = moveSlot.maxpp;
    {
        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => {
                debug_elog!("[MYSTERYBERRY] pokemon_at_mut returned None for {:?}", pokemon_pos);
                return EventResult::Continue;
            }
        };

        debug_elog!("[MYSTERYBERRY] Modifying Pokemon '{}' at position {:?}", pokemon_mut.name, pokemon_pos);
        if move_slot_index < pokemon_mut.move_slots.len() {
            let move_slot = &mut pokemon_mut.move_slots[move_slot_index];
            let old_pp = move_slot.pp;
            let new_pp = move_slot.pp.saturating_add(5).min(move_slot.maxpp);
            let move_id = move_slot.id.clone();
            move_slot.pp = new_pp;
            debug_elog!("[MYSTERYBERRY] PP restored: {} -> {} (maxpp={})", old_pp, new_pp, move_slot.maxpp);
            debug_elog!("[MYSTERYBERRY] After modification, move_slot.pp = {}", move_slot.pp);

            // Also sync to base_move_slots so clearVolatile preserves PP
            // (In JS, moveSlots and baseMoveSlots share the same MoveSlot objects)
            if let Some(base_slot) = pokemon_mut.base_move_slots.iter_mut().find(|s| s.id == move_id) {
                base_slot.pp = new_pp;
                debug_elog!("[MYSTERYBERRY] Synced to base_move_slots: pp = {}", new_pp);
            }
        } else {
            debug_elog!("[MYSTERYBERRY] move_slot_index {} >= len {}", move_slot_index, pokemon_mut.move_slots.len());
        }
    }

    // Debug: Verify the modification persisted by re-reading the Pokemon
    {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => {
                debug_elog!("[MYSTERYBERRY] VERIFY: pokemon_at returned None");
                return EventResult::Continue;
            }
        };
        debug_elog!("[MYSTERYBERRY] VERIFY: Pokemon '{}' move_slots[{}].pp = {}",
            pokemon.name,
            move_slot_index,
            pokemon.move_slots.get(move_slot_index).map(|m| m.pp).unwrap_or(255));
    }

    // this.add('-activate', pokemon, 'item: Mystery Berry', moveSlot.move);
    battle.add("-activate", &[
        pokemon_ident.as_str().into(),
        "item: Mystery Berry".into(),
        move_name.into()
    ]);

    EventResult::Continue
}

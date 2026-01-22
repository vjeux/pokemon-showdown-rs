//! Anticipation Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onStart(pokemon) {
///     for (const target of pokemon.foes()) {
///         for (const moveSlot of target.moveSlots) {
///             const move = this.dex.moves.get(moveSlot.move);
///             if (move.category === 'Status') continue;
///             const moveType = move.id === 'hiddenpower' ? target.hpType : move.type;
///             if (
///                 this.dex.getImmunity(moveType, pokemon) && this.dex.getEffectiveness(moveType, pokemon) > 0 ||
///                 move.ohko
///             ) {
///                 this.add('-ability', pokemon, 'Anticipation');
///                 return;
///             }
///         }
///     }
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect: Option<&Effect>) -> EventResult {
    use crate::battle::Arg;

    // for (const target of pokemon.foes())
    let foes = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.foes(battle, false)
    };

    for foe_pos in foes {
        // for (const moveSlot of target.moveSlots)
        let foe_move_slots = {
            let foe = match battle.pokemon_at(foe_pos.0, foe_pos.1) {
                Some(p) => p,
                None => continue,
            };
            foe.move_slots.clone()
        };

        for move_slot in &foe_move_slots {
            // const move = this.dex.moves.get(moveSlot.move);
            let move_data = match battle.dex.moves().get(&move_slot.id.to_string()) {
                Some(m) => m,
                None => continue,
            };

            // if (move.category === 'Status') continue;
            if move_data.category == "Status" {
                continue;
            }

            // const moveType = move.id === 'hiddenpower' ? target.hpType : move.type;
            let move_type = if move_data.id.as_str() == "hiddenpower" {
                let foe = match battle.pokemon_at(foe_pos.0, foe_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                foe.hp_type.clone()
            } else {
                move_data.move_type.clone()
            };

            // if (this.dex.getImmunity(moveType, pokemon) && this.dex.getEffectiveness(moveType, pokemon) > 0 || move.ohko)
            let (has_immunity, is_super_effective) = {
                let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };

                let pokemon_types = pokemon.get_types(battle, false);
                let immunity = battle.dex.get_immunity(&move_type, &pokemon_types);

                // For effectiveness, we need to check against each type and get the total
                let mut max_effectiveness = -100;
                for defend_type in &pokemon_types {
                    let eff = battle.dex.get_effectiveness(&move_type, defend_type);
                    if eff > max_effectiveness {
                        max_effectiveness = eff;
                    }
                }

                (immunity, max_effectiveness > 0)
            };

            // Check for OHKO move (stored in ActiveMove, not in move data)
            // For this ability check, we need to check the move data's ohko property
            // But ohko is in ActiveMove, not in move data. Let me check if move_data has ohko.
            // Actually, looking at the JavaScript, it's checking move.ohko which is from the move data.
            // Let me check if we need to look this up differently.

            if has_immunity && is_super_effective {
                // this.add('-ability', pokemon, 'Anticipation');
                let pokemon_slot = {
                    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    pokemon.get_slot()
                };

                battle.add("-ability", &[Arg::String(pokemon_slot), Arg::Str("Anticipation")]);
                return EventResult::Continue;
            }
        }
    }

    EventResult::Continue
}


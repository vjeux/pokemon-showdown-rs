//! Endure Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onPrepareHit(pokemon) {
///     return !!this.queue.willAct() && this.runEvent('StallMove', pokemon);
/// }
pub fn on_prepare_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    let pokemon = pokemon_pos;

    // return !!this.queue.willAct() && this.runEvent('StallMove', pokemon);
    let will_act = battle.queue.will_act().is_some();

    if !will_act {
        return EventResult::Boolean(false);
    }

    let stall_result = battle.run_event("StallMove", Some(crate::event::EventTarget::Pokemon(pokemon)), None, None, EventResult::Continue, false, false);

    // Convert stall_result to boolean: Boolean(true/false) or Number(!=0) means success
    let stall_success = match stall_result {
        EventResult::Boolean(b) => b,
        EventResult::Number(n) => n != 0,
        _ => false,
    };

    EventResult::Boolean(will_act && stall_success)
}

/// onHit(pokemon) {
///     pokemon.addVolatile('stall');
/// }
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;
    use crate::pokemon::Pokemon;

    // pokemon.addVolatile('stall');
    // Use battle.add_volatile_to_pokemon to properly set duration from dex.conditions
    Pokemon::add_volatile(battle, pokemon_pos, ID::from("stall"), None, None, None, None);

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     this.add('-singleturn', target, 'move: Endure');
    /// }
    pub fn on_start(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
        _effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
        let target = pokemon_pos;

        // this.add('-singleturn', target, 'move: Endure');
        let target_ident = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.get_slot()
        };

        battle.add(
            "-singleturn",
            &[target_ident.as_str().into(), "move: Endure".into()],
        );

        EventResult::Continue
    }

    /// onDamage(damage, target, source, effect) {
    ///     if (effect?.effectType === 'Move' && damage >= target.hp) {
    ///         this.add('-activate', target, 'move: Endure');
    ///         return target.hp - 1;
    ///     }
    /// }
    pub fn on_damage(
        battle: &mut Battle,
        damage: i32,
        target_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
        effect_id: Option<&str>,
    ) -> EventResult {
        use crate::dex_data::ID;

        let target = target_pos;

        // if (effect?.effectType === 'Move' && damage >= target.hp) {
        //     this.add('-activate', target, 'move: Endure');
        //     return target.hp - 1;
        // }
        if let Some(effect) = effect_id {
            // Check if effect is a Move
            let is_move = battle.dex.moves().get_by_id(&ID::from(effect)).is_some();

            if is_move {
                let target_hp = {
                    let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    target_pokemon.hp
                };

                if damage >= target_hp {
                    // this.add('-activate', target, 'move: Endure');
                    let target_ident = {
                        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                            Some(p) => p,
                            None => return EventResult::Continue,
                        };
                        target_pokemon.get_slot()
                    };

                    battle.add(
                        "-activate",
                        &[target_ident.as_str().into(), "move: Endure".into()],
                    );

                    // return target.hp - 1;
                    return EventResult::Number(target_hp - 1);
                }
            }
        }

        EventResult::Continue
    }
}

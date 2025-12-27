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
pub fn on_prepare_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let pokemon = pokemon_pos;

    // return !!this.queue.willAct() && this.runEvent('StallMove', pokemon);
    let will_act = battle.queue.will_act();

    if !will_act {
        return EventResult::Bool(false);
    }

    let stall_result = battle.run_event("StallMove", pokemon, None, None);

    EventResult::Bool(stall_result)
}

/// onHit(pokemon) {
///     pokemon.addVolatile('stall');
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = pokemon_pos;

    // pokemon.addVolatile('stall');
    let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    pokemon_pokemon.add_volatile(ID::from("stall"));

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     this.add('-singleturn', target, 'move: Endure');
    /// }
    pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // this.add('-singleturn', target, 'move: Endure');
        let target_arg = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(target_pokemon)
        };

        battle.add("-singleturn", &[target_arg, "move: Endure".into()]);

        EventResult::Continue
    }

    /// onDamage(damage, target, source, effect) {
    ///     if (effect?.effectType === 'Move' && damage >= target.hp) {
    ///         this.add('-activate', target, 'move: Endure');
    ///         return target.hp - 1;
    ///     }
    /// }
    pub fn on_damage(battle: &mut Battle, damage: i32, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        use crate::dex_data::ID;

        let target = target_pos;

        // if (effect?.effectType === 'Move' && damage >= target.hp) {
        //     this.add('-activate', target, 'move: Endure');
        //     return target.hp - 1;
        // }
        if let Some(effect) = effect_id {
            // Check if effect is a Move
            let is_move = battle.dex.get_move_by_id(&ID::from(effect)).is_some();

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
                    let target_arg = {
                        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                            Some(p) => p,
                            None => return EventResult::Continue,
                        };
                        crate::battle::Arg::from(target_pokemon)
                    };

                    battle.add("-activate", &[target_arg, "move: Endure".into()]);

                    // return target.hp - 1;
                    return EventResult::Int(target_hp - 1);
                }
            }
        }

        EventResult::Continue
    }
}

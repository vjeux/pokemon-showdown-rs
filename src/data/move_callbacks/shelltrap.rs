//! Shell Trap Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// priorityChargeCallback(pokemon) {
///     pokemon.addVolatile('shelltrap');
/// }
pub fn priority_charge_callback(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    // priorityChargeCallback(pokemon) {
    //     pokemon.addVolatile('shelltrap');
    // }
    let pokemon = pokemon_pos;

    {


        let pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {


            Some(p) => p,


            None => return EventResult::Continue,


        };


        pokemon.add_volatile(ID::from("shelltrap"));


    }

    EventResult::Continue
}

/// onTryMove(pokemon) {
///     if (!pokemon.volatiles['shelltrap']?.gotHit) {
///         this.attrLastMove('[still]');
///         this.add('cant', pokemon, 'Shell Trap', 'Shell Trap');
///         return null;
///     }
/// }
pub fn on_try_move(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    // onTryMove(pokemon) {
    //     if (!pokemon.volatiles['shelltrap']?.gotHit) {
    //         this.attrLastMove('[still]');
    //         this.add('cant', pokemon, 'Shell Trap', 'Shell Trap');
    //         return null;
    //     }
    // }
    let pokemon = source_pos;

    // if (!pokemon.volatiles['shelltrap']?.gotHit) {
    let got_hit = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        if let Some(volatile) = pokemon_pokemon.volatiles.get(&ID::from("shelltrap")) {
            volatile.data.get("gotHit")
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
        } else {
            false
        }
    };

    if !got_hit {
        // this.attrLastMove('[still]');
        battle.attr_last_move(&["[still]"]);

        // this.add('cant', pokemon, 'Shell Trap', 'Shell Trap');
        let pokemon_arg = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(pokemon_pokemon)
        };

        battle.add("cant", &[
            pokemon_arg,
            "Shell Trap".into(),
            "Shell Trap".into(),
        ]);

        // return null;
        return EventResult::Stop;
    }

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     this.add('-singleturn', pokemon, 'move: Shell Trap');
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // onStart(pokemon) {
        //     this.add('-singleturn', pokemon, 'move: Shell Trap');
        // }
        let pokemon = pokemon_pos;

        let pokemon_arg = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(pokemon_pokemon)
        };

        battle.add("-singleturn", &[
            pokemon_arg,
            "move: Shell Trap".into(),
        ]);

        EventResult::Continue
    }

    /// onHit(pokemon, source, move) {
    ///     if (!pokemon.isAlly(source) && move.category === 'Physical') {
    ///         this.effectState.gotHit = true;
    ///         const action = this.queue.willMove(pokemon);
    ///         if (action) {
    ///             this.queue.prioritizeAction(action);
    ///         }
    ///     }
    /// }
    pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
        use crate::dex_data::ID;

        // onHit(pokemon, source, move) {
        //     if (!pokemon.isAlly(source) && move.category === 'Physical') {
        //         this.effectState.gotHit = true;
        //         const action = this.queue.willMove(pokemon);
        //         if (action) {
        //             this.queue.prioritizeAction(action);
        //         }
        //     }
        // }
        let pokemon = pokemon_pos;
        let source = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (!pokemon.isAlly(source) && move.category === 'Physical') {
        let is_ally = battle.is_ally(pokemon, source);
        let is_physical = {
            let active_move = match &battle.active_move {
                Some(active_move) => active_move,
                None => return EventResult::Continue,
            };
            active_move.category == "Physical"
        };

        if !is_ally && is_physical {
            // this.effectState.gotHit = true;
            if let Some(ref mut effect_state) = battle.current_effect_state {
                effect_state.data.insert("gotHit".to_string(), serde_json::json!(true));
            }

            // const action = this.queue.willMove(pokemon);
            // if (action) {
            //     this.queue.prioritizeAction(action);
            // }
            let action = battle.queue_will_move(pokemon);
            if action.is_some() {
                battle.queue_prioritize_action(pokemon);
            }
        }

        EventResult::Continue
    }
}

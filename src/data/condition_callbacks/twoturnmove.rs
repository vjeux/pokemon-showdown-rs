//! Twoturnmove Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::battle::Effect;

use crate::event::EventResult;
use crate::dex_data::ID;

/// onStart
/// JavaScript source (data/conditions.ts):
/// ```js
/// onStart(attacker, defender, effect) {
///     // ("attacker" is the Pokemon using the two turn move and the Pokemon this condition is being applied to)
///     this.effectState.move = effect.id;
///     attacker.addVolatile(effect.id);
///     // lastMoveTargetLoc is the location of the originally targeted slot before any redirection
///     // note that this is not updated for moves called by other moves
///     // i.e. if Dig is called by Metronome, lastMoveTargetLoc will still be the user's location
///     let moveTargetLoc: number = attacker.lastMoveTargetLoc!;
///     if (effect.sourceEffect && this.dex.moves.get(effect.id).target !== 'self') {
///         // this move was called by another move such as Metronome
///         // and needs a random target to be determined this turn
///         // it will already have one by now if there is any valid target
///         // but if there isn't one we need to choose a random slot now
///         if (defender.fainted) {
///             defender = this.sample(attacker.foes(true));
///         }
///         moveTargetLoc = attacker.getLocOf(defender);
///     }
///     attacker.volatiles[effect.id].targetLoc = moveTargetLoc;
///     this.attrLastMove('[still]');
///     // Run side-effects normally associated with hitting (e.g., Protean, Libero)
///     this.runEvent('PrepareHit', attacker, defender, effect);
/// }
/// ```
pub fn on_start(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
    _effect: Option<&Effect>,
) -> EventResult {
    debug_elog!("[TWOTURNMOVE_ONSTART] turn={}, pokemon=({},{})", battle.turn, pokemon_pos.0, pokemon_pos.1);

    // this.effectState.move = effect.id;
    // Get the move ID from battle.event.effect
    let move_id = battle.event.as_ref()
        .and_then(|e| e.effect.as_ref())
        .map(|eff| eff.id.clone());

    debug_elog!("[TWOTURNMOVE_ONSTART] move_id={:?}", move_id.as_ref().map(|id| id.as_str()));

    if let Some(ref move_id_val) = move_id {
        // Store move ID in twoturnmove volatile's effectState.data
        // JavaScript: this.effectState.move = effect.id;
        // In Rust, current_effect_state is the volatile's state (set by dispatch_single_event)
        // We must modify current_effect_state, which will be copied back to the volatile
        debug_elog!("[TWOTURNMOVE_ONSTART] Storing move_id={} in effect_state", move_id_val.as_str());
        battle.with_effect_state(|state| {
            state.move_id = Some(move_id_val.to_string());
            debug_elog!("[TWOTURNMOVE_ONSTART] Stored successfully");
        }).unwrap_or_else(|| {
            debug_elog!("[TWOTURNMOVE_ONSTART] WARNING: current_effect_state is None!");
        });

        // attacker.addVolatile(effect.id);
        // Add a volatile for the specific move (e.g., "dig", "fly", "solarbeam")
        //
        // IMPORTANT: JavaScript does NOT pass a source parameter explicitly here,
        // BUT in JavaScript's addVolatile, when source is not passed, it inherits
        // from this.battle.event.source (see pokemon.ts lines 1945-1949):
        //   if (this.battle.event) {
        //       if (!source) source = this.battle.event.source;
        //   }
        //   if (!source) source = this;
        //
        // When twoturnmove.onStart runs, this.battle.event.source = the source
        // passed to the original addVolatile('twoturnmove', target) call, which
        // is the grabbed target (e.g., Ribombee for Sky Drop).
        //
        // This is critical for Sky Drop's onFoeBeforeMove which checks:
        //   if (attacker === this.effectState.source) { return null; }
        // The grabbed Pokemon (Ribombee) is stored as effectState.source,
        // so when Ribombee tries to move, attacker === Ribombee is TRUE,
        // and the move is nullified.
        //
        // Get the event source (the grabbed target) to pass to add_volatile
        let event_source = battle.event.as_ref().and_then(|e| e.source);
        debug_elog!("[TWOTURNMOVE_ONSTART] About to add_volatile for move='{}', source={:?} (inherited from event)", move_id_val.as_str(), event_source);
        crate::pokemon::Pokemon::add_volatile(battle, pokemon_pos, move_id_val.clone(), event_source, None, None, None);
        debug_elog!("[TWOTURNMOVE_ONSTART] Returned from add_volatile");

        // JavaScript: let moveTargetLoc: number = attacker.lastMoveTargetLoc!;
        let mut move_target_loc = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.last_move_target_loc.unwrap_or(0)
        };

        // JavaScript: if (effect.sourceEffect && this.dex.moves.get(effect.id).target !== 'self') {
        let source_effect_exists = battle.active_move.as_ref()
            .and_then(|m| m.source_effect.as_ref())
            .is_some();

        if source_effect_exists {
            // Check if the move's target type is not 'self'
            let move_target_type = battle.dex.moves().get(move_id_val.as_str())
                .map(|m| m.target.as_str())
                .unwrap_or("");

            if move_target_type != "self" {
                // Get defender position from battle.event.source
                let mut defender_pos = battle.event.as_ref()
                    .and_then(|e| e.source);

                // JavaScript: if (defender.fainted) { defender = this.sample(attacker.foes(true)); }
                if let Some(def_pos) = defender_pos {
                    let defender_fainted = {
                        let defender = match battle.pokemon_at(def_pos.0, def_pos.1) {
                            Some(p) => p,
                            None => return EventResult::Continue,
                        };
                        defender.fainted
                    };

                    if defender_fainted {
                        // Sample a random foe
                        // JavaScript: defender = this.sample(attacker.foes(true));
                        let foes = {
                            let attacker = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                                Some(p) => p,
                                None => return EventResult::Continue,
                            };
                            attacker.foes(battle, true)
                        };
                        if !foes.is_empty() {
                            let random_foe = battle.sample(&foes).copied();
                            defender_pos = random_foe;
                        }
                    }
                }

                // JavaScript: moveTargetLoc = attacker.getLocOf(defender);
                if let Some(def_pos) = defender_pos {
                    let attacker = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    move_target_loc = attacker.get_loc_of(def_pos.0, def_pos.1, 1);
                }
            }
        }

        // JavaScript: attacker.volatiles[effect.id].targetLoc = moveTargetLoc;
        // Store targetLoc in the move volatile's effectState.data
        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        if let Some(state) = pokemon_mut.volatiles.get_mut(move_id_val) {
            state.target_loc = Some(move_target_loc as i32);
        }
    }

    // JavaScript: this.attrLastMove('[still]');
    battle.attr_last_move(&["[still]"]);

    // JavaScript: this.runEvent('PrepareHit', attacker, defender, effect);
    // Run side-effects normally associated with hitting (e.g., Protean, Libero)
    let defender_pos = battle.event.as_ref().and_then(|e| e.source);
    battle.run_event(
        "PrepareHit",
        Some(crate::event::EventTarget::Pokemon(pokemon_pos)),
        defender_pos,
        move_id.as_ref().map(|id| Effect::move_(id.clone())).as_ref(),
        EventResult::Continue,
        false,
        false,
    );

    EventResult::Continue
}

/// onEnd
/// JavaScript source (data/conditions.ts):
/// ```js
/// onEnd(target) {
///     target.removeVolatile(this.effectState.move);
/// }
/// ```
pub fn on_end(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    debug_elog!("[TWOTURNMOVE_ONEND] Called for pokemon=({},{}), turn={}",
        pokemon_pos.0, pokemon_pos.1, battle.turn);

    // target.removeVolatile(this.effectState.move);
    // Get the move ID from effectState using with_effect_state_ref
    // JavaScript: this.effectState.move
    let move_id = battle.with_effect_state_ref(|state| {
        debug_elog!("[TWOTURNMOVE_ONEND] effect_state.move_id = {:?}", state.move_id);
        state.move_id.as_ref().map(|s| ID::from(s.as_str()))
    }).flatten();

    debug_elog!("[TWOTURNMOVE_ONEND] move_id to remove = {:?}", move_id.as_ref().map(|id| id.as_str()));

    // Remove the volatile for the specific move (e.g., "dig", "fly", "solarbeam")
    if let Some(id) = move_id {
        debug_elog!("[TWOTURNMOVE_ONEND] Calling remove_volatile for '{}'", id.as_str());
        crate::pokemon::Pokemon::remove_volatile(battle, pokemon_pos, &id);
        debug_elog!("[TWOTURNMOVE_ONEND] remove_volatile returned");
    } else {
        debug_elog!("[TWOTURNMOVE_ONEND] No move_id found, not removing any volatile");
    }

    EventResult::Continue
}

/// onLockMove
/// JavaScript source (data/conditions.ts):
/// ```js
/// onLockMove() {
///     return this.effectState.move;
/// }
/// ```
pub fn on_lock_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    debug_elog!("[TWOTURNMOVE_ONLOCKMOVE] Called for pokemon=({},{}), turn={}", pokemon_pos.0, pokemon_pos.1, battle.turn);

    // return this.effectState.move;
    // Get the move ID from effectState using with_effect_state_ref
    // JavaScript: this.effectState.move
    let move_id = battle.with_effect_state_ref(|state| {
        debug_elog!("[TWOTURNMOVE_ONLOCKMOVE] Found effect state!");
        debug_elog!("[TWOTURNMOVE_ONLOCKMOVE] State move_id: {:?}", state.move_id);
        debug_elog!("[TWOTURNMOVE_ONLOCKMOVE] State duration: {:?}", state.duration);
        state.move_id.clone()
    }).flatten();

    debug_elog!("[TWOTURNMOVE_ONLOCKMOVE] move_id={:?}", move_id);
    match move_id {
        Some(id) => {
            debug_elog!("[TWOTURNMOVE_ONLOCKMOVE] Returning String({})", id);
            EventResult::String(id)
        }
        None => {
            debug_elog!("[TWOTURNMOVE_ONLOCKMOVE] No move_id, returning Continue");
            EventResult::Continue
        }
    }
}

/// onMoveAborted
/// JavaScript source (data/conditions.ts):
/// ```js
/// onMoveAborted(pokemon) {
///     pokemon.removeVolatile('twoturnmove');
/// }
/// ```
pub fn on_move_aborted(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
    _active_move: Option<&crate::battle_actions::ActiveMove>,
) -> EventResult {
    // pokemon.removeVolatile('twoturnmove');
    crate::pokemon::Pokemon::remove_volatile(battle, pokemon_pos, &ID::from("twoturnmove"));

    EventResult::Continue
}


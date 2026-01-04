//! Twoturnmove Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
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
) -> EventResult {
    // this.effectState.move = effect.id;
    // Get the move ID from battle.current_event.effect
    let move_id = battle.current_event.as_ref()
        .and_then(|e| e.effect.as_ref())
        .map(|id| id.clone());

    if let Some(ref move_id_val) = move_id {
        // Store move ID in twoturnmove volatile's effectState.data
        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let twoturnmove_id = ID::from("twoturnmove");
        if let Some(state) = pokemon_mut.volatiles.get_mut(&twoturnmove_id) {
            state.data.insert("move".to_string(), serde_json::json!(move_id_val.as_str()));
        }

        // attacker.addVolatile(effect.id);
        // Add a volatile for the specific move (e.g., "dig", "fly", "solarbeam")
        crate::pokemon::Pokemon::add_volatile(battle, pokemon_pos, move_id_val.clone(), None, None, None, None);
    }

    // TODO: Handle lastMoveTargetLoc and targetLoc storage
    // TODO: this.attrLastMove('[still]');
    // TODO: this.runEvent('PrepareHit', attacker, defender, effect);

    eprintln!("[TWOTURNMOVE_ON_START] Called for {:?}, stored move={:?}, added move volatile - TODO: lastMoveTargetLoc, attrLastMove, PrepareHit event", pokemon_pos, move_id);

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
    // target.removeVolatile(this.effectState.move);
    // Get the move ID from the twoturnmove volatile's effectState.data
    let move_id = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let twoturnmove_id = ID::from("twoturnmove");
        pokemon.volatiles.get(&twoturnmove_id)
            .and_then(|v| v.data.get("move"))
            .and_then(|m| m.as_str())
            .map(|s| ID::from(s))
    };

    // Remove the volatile for the specific move (e.g., "dig", "fly", "solarbeam")
    if let Some(id) = move_id {
        crate::pokemon::Pokemon::remove_volatile(battle, pokemon_pos, &id);
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
    // return this.effectState.move;
    // Get the move ID from the twoturnmove volatile's effectState.data
    let move_id = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let twoturnmove_id = ID::from("twoturnmove");
        pokemon.volatiles.get(&twoturnmove_id)
            .and_then(|v| v.data.get("move"))
            .and_then(|m| m.as_str())
            .map(|s| s.to_string())
    };

    match move_id {
        Some(id) => EventResult::String(id),
        None => EventResult::Continue,
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
) -> EventResult {
    // pokemon.removeVolatile('twoturnmove');
    crate::pokemon::Pokemon::remove_volatile(battle, pokemon_pos, &ID::from("twoturnmove"));

    EventResult::Continue
}


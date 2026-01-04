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
    // TODO: Store move ID in effectState data

    // attacker.addVolatile(effect.id);
    // TODO: Need to get effect.id and add volatile to attacker
    // The effect should be the move ID (e.g., "dig", "fly", "solarbeam")

    // TODO: Handle lastMoveTargetLoc and targetLoc storage
    // TODO: this.attrLastMove('[still]');
    // TODO: this.runEvent('PrepareHit', attacker, defender, effect);

    eprintln!("[TWOTURNMOVE_ON_START] Called for {:?} - TODO: Full implementation needs effectState and event infrastructure", pokemon_pos);

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
    // TODO: Need to get move ID from effectState to know which volatile to remove
    // For now, we can't implement this without accessing the effectState

    eprintln!("[TWOTURNMOVE_ON_END] Called for {:?} - TODO: Remove volatile (need effectState.move)", pokemon_pos);

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
    _battle: &mut Battle,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    // return this.effectState.move;
    // TODO: Need to access effectState.move to return the locked move ID
    // This should return the move ID string that the Pokemon is locked into

    eprintln!("[TWOTURNMOVE_ON_LOCK_MOVE] TODO: Return effectState.move");

    EventResult::Continue
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


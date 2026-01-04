//! Futuremove Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::battle::Arg;
use crate::event::EventResult;
use crate::dex_data::ID;

/// onStart
/// JavaScript source (data/conditions.ts):
/// ```js
/// onStart(target) {
///     this.effectState.targetSlot = target.getSlot();
///     this.effectState.endingTurn = (this.turn - 1) + 2;
///     if (this.effectState.endingTurn >= 254) {
///         this.hint(`In Gen 8+, Future attacks will never resolve when used on the 255th turn or later.`);
///     }
/// }
/// ```
pub fn on_start(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // this.effectState.targetSlot = target.getSlot();
    let target_slot = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    // this.effectState.endingTurn = (this.turn - 1) + 2;
    let ending_turn = (battle.turn - 1) + 2;

    // Store in slot condition's effectState
    // TODO: Need access to slot condition data storage
    // For now, this data needs to be stored but the infrastructure isn't available yet
    eprintln!("[FUTUREMOVE] Storing targetSlot={}, endingTurn={}", target_slot, ending_turn);

    // if (this.effectState.endingTurn >= 254)
    if ending_turn >= 254 {
        // this.hint(`In Gen 8+, Future attacks will never resolve when used on the 255th turn or later.`);
        battle.hint("In Gen 8+, Future attacks will never resolve when used on the 255th turn or later.", false, None);
    }

    EventResult::Continue
}

/// onResidual
/// JavaScript source (data/conditions.ts):
/// ```js
/// onResidualOrder: 3,
/// onResidual(target: Pokemon) {
///     if (this.getOverflowedTurnCount() < this.effectState.endingTurn) return;
///     target.side.removeSlotCondition(this.getAtSlot(this.effectState.targetSlot), 'futuremove');
/// }
/// ```
pub fn on_residual(
    _battle: &mut Battle,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    // if (this.getOverflowedTurnCount() < this.effectState.endingTurn) return;
    // TODO: Need getOverflowedTurnCount() and access to effectState.endingTurn
    // TODO: Need target.side.removeSlotCondition() implementation
    eprintln!("[FUTUREMOVE] onResidual - TODO: Check turn count and remove slot condition");

    EventResult::Continue
}

/// onEnd
/// JavaScript source (data/conditions.ts):
/// ```js
/// onEnd(target) {
///     const data = this.effectState;
///     // time's up; time to hit! :D
///     const move = this.dex.moves.get(data.move);
///     if (target.fainted || target === data.source) {
///         this.hint(`${move.name} did not hit because the target is ${(target.fainted ? 'fainted' : 'the user')}.`);
///         return;
///     }
///
///     this.add('-end', target, 'move: ' + move.name);
///     target.removeVolatile('Protect');
///     target.removeVolatile('Endure');
///
///     if (data.source.hasAbility('infiltrator') && this.gen >= 6) {
///         data.moveData.infiltrates = true;
///     }
///     if (data.source.hasAbility('normalize') && this.gen >= 6) {
///         data.moveData.type = 'Normal';
///     }
///     const hitMove = new this.dex.Move(data.moveData) as ActiveMove;
///
///     this.actions.trySpreadMoveHit([target], data.source, hitMove, true);
///     if (data.source.isActive && data.source.hasItem('lifeorb') && this.gen >= 5) {
///         this.singleEvent('AfterMoveSecondarySelf', data.source.getItem(), data.source.itemState, data.source, target, data.source.getItem());
///     }
///     this.activeMove = null;
///
///     this.checkWin();
/// }
/// ```
pub fn on_end(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // const data = this.effectState;
    // TODO: Need access to slot condition effectState data (move, source, moveData, etc.)

    // Check if target fainted
    let target_fainted = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.fainted
    };

    if target_fainted {
        // this.hint(`${move.name} did not hit because the target is fainted.`);
        // TODO: Implement battle.hint() and get move name from effectState
        eprintln!("[FUTUREMOVE] Target fainted, future move will not hit");
        return EventResult::Continue;
    }

    // TODO: Check if target === data.source (same pokemon)
    // TODO: this.add('-end', target, 'move: ' + move.name);

    // target.removeVolatile('Protect');
    crate::pokemon::Pokemon::remove_volatile(battle, pokemon_pos, &ID::from("protect"));

    // target.removeVolatile('Endure');
    crate::pokemon::Pokemon::remove_volatile(battle, pokemon_pos, &ID::from("endure"));

    // TODO: Check source abilities (infiltrator, normalize)
    // TODO: Create hitMove from moveData
    // TODO: this.actions.trySpreadMoveHit([target], data.source, hitMove, true);
    // TODO: Handle Life Orb damage
    // TODO: this.checkWin();

    eprintln!("[FUTUREMOVE] onEnd - TODO: Execute future move hit with full logic");

    EventResult::Continue
}


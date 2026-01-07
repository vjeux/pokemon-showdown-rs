//! BattleActions::afterMoveSecondaryEvent - Fire AfterMoveSecondary events
//!
//! 1:1 port of afterMoveSecondaryEvent from battle-actions.ts

use crate::*;
use crate::event::EventResult;

/// Fire AfterMoveSecondary events after a move hits
/// Equivalent to battle-actions.ts afterMoveSecondaryEvent()
///
/// afterMoveSecondaryEvent(targets: Pokemon[], pokemon: Pokemon, move: ActiveMove) {
///     // console.log(`${targets}, ${pokemon}, ${move}`)
///     if (!(move.hasSheerForce && pokemon.hasAbility('sheerforce'))) {
///         this.battle.singleEvent('AfterMoveSecondary', move, null, targets[0], pokemon, move);
///         this.battle.runEvent('AfterMoveSecondary', targets, pokemon, move);
///     }
///     return undefined;
/// }
pub fn after_move_secondary_event(
    battle: &mut Battle,
    targets: &[(usize, usize)],
    attacker_pos: (usize, usize),
    active_move: &crate::battle_actions::ActiveMove,
) {
    // if (!(move.hasSheerForce && pokemon.hasAbility('sheerforce'))) {
    // Check sheer force in two phases to avoid borrow issues
    let has_sheer_force_ability = {
        if let Some(pokemon) = battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
            pokemon.has_ability(battle, &["sheerforce"])
        } else {
            return;
        }
    };

    let sheer_force_active = active_move.has_sheer_force && has_sheer_force_ability;

    if !sheer_force_active {
        //     this.battle.singleEvent('AfterMoveSecondary', move, null, targets[0], pokemon, move);
        if let Some(&first_target) = targets.first() {
            battle.single_event(
                "AfterMoveSecondary",
                &crate::battle::Effect::move_(active_move.id.clone()),
                Some(first_target),
                Some(attacker_pos),
                None,
                None,
            );
        }

        //     this.battle.runEvent('AfterMoveSecondary', targets, pokemon, move);
        // Note: runEvent in JavaScript takes targets array, but Rust version takes single target
        // We need to fire the event for each target
        for &target in targets {
            battle.run_event("AfterMoveSecondary", Some(crate::event::EventTarget::Pokemon(target)), Some(attacker_pos), Some(&active_move.id), EventResult::Continue, false, false);
        }
    }

    // return undefined;
}

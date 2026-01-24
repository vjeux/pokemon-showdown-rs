//! Rebound Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, source, move) {
///     if (this.effectState.target.activeTurns) return;
///
///     if (target === source || move.hasBounced || !move.flags['reflectable'] || target.isSemiInvulnerable()) {
///         return;
///     }
///     const newMove = this.dex.getActiveMove(move.id);
///     newMove.hasBounced = true;
///     newMove.pranksterBoosted = false;
///     this.actions.useMove(newMove, target, { target: source });
///     return null;
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult { let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
    use crate::pokemon::Pokemon;

    // if (this.effectState.target.activeTurns) return;
    // Rebound only works on the turn the Pokemon switches in (activeTurns == 0)
    let active_turns = {
        let rebound_holder = match battle.effect_state.borrow().target {
            Some(holder) => holder,
            None => return EventResult::Continue,
        };
        let pokemon = match battle.pokemon_at(rebound_holder.0, rebound_holder.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.active_turns
    };

    if active_turns > 0 {
        return EventResult::Continue;
    }

    // if (target === source || move.hasBounced || !move.flags['reflectable'] || target.isSemiInvulnerable()) {
    //     return;
    // }
    if target_pos == source_pos {
        return EventResult::Continue;
    }

    let (has_bounced, is_reflectable, target_semi_invulnerable) = {
        let active_move = match &battle.active_move {
            Some(m) => m,
            None => return EventResult::Continue,
        };

        let target_semi_invuln = Pokemon::is_semi_invulnerable(battle, target_pos);

        (active_move.has_bounced, active_move.flags.reflectable, target_semi_invuln)
    };

    if has_bounced || !is_reflectable || target_semi_invulnerable {
        return EventResult::Continue;
    }

    // const newMove = this.dex.getActiveMove(move.id);
    // newMove.hasBounced = true;
    // newMove.pranksterBoosted = false;
    // Set has_bounced and pranksterBoosted = false on current active move
    if let Some(ref mut active_move) = battle.active_move {
        active_move.has_bounced = true;
        active_move.prankster_boosted = false;
    }

    // this.actions.useMove(newMove, target, { target: source });
    // Reflect the move: Rebound holder (target) uses the move against the original source
    let move_data = match battle.dex.moves().get(move_id).cloned() {
        Some(m) => m,
        None => return EventResult::Continue,
    };
    crate::battle_actions::use_move(
        battle,
        &move_data,
        target_pos,        // Rebound holder becomes the user
        Some(source_pos),  // Original source becomes the target
        None,              // No source effect
        None,              // Not a Z-move
        None,              // Not a Max move
    );

    // return null;
    EventResult::Null
}

/// onAllyTryHitSide(target, source, move) {
///     if (this.effectState.target.activeTurns) return;
///
///     if (target.isAlly(source) || move.hasBounced || !move.flags['reflectable'] || target.isSemiInvulnerable()) {
///         return;
///     }
///     const newMove = this.dex.getActiveMove(move.id);
///     newMove.hasBounced = true;
///     newMove.pranksterBoosted = false;
///     this.actions.useMove(newMove, this.effectState.target, { target: source });
///     move.hasBounced = true; // only bounce once in free-for-all battles
///     return null;
/// }
pub fn on_ally_try_hit_side(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult { let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
    use crate::pokemon::Pokemon;

    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (this.effectState.target.activeTurns) return;
    // Rebound only works on the turn the Pokemon switches in (activeTurns == 0)
    let active_turns = {
        let rebound_holder = match battle.effect_state.borrow().target {
            Some(holder) => holder,
            None => return EventResult::Continue,
        };
        let pokemon = match battle.pokemon_at(rebound_holder.0, rebound_holder.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.active_turns
    };

    if active_turns > 0 {
        return EventResult::Continue;
    }

    // if (target.isAlly(source) || move.hasBounced || !move.flags['reflectable'] || target.isSemiInvulnerable()) {
    //     return;
    // }
    let is_ally = battle.is_ally(target, source);
    if is_ally {
        return EventResult::Continue;
    }

    let (has_bounced, is_reflectable, target_semi_invulnerable) = {
        let active_move = match &battle.active_move {
            Some(m) => m,
            None => return EventResult::Continue,
        };

        let target_semi_invuln = Pokemon::is_semi_invulnerable(battle, target);

        (active_move.has_bounced, active_move.flags.reflectable, target_semi_invuln)
    };

    if has_bounced || !is_reflectable || target_semi_invulnerable {
        return EventResult::Continue;
    }

    // const newMove = this.dex.getActiveMove(move.id);
    // newMove.hasBounced = true;
    // newMove.pranksterBoosted = false;
    // move.hasBounced = true; // only bounce once in free-for-all battles
    // Set has_bounced and pranksterBoosted = false on current active move
    if let Some(ref mut active_move) = battle.active_move {
        active_move.has_bounced = true;
        active_move.prankster_boosted = false;
    }

    // this.actions.useMove(newMove, this.effectState.target, { target: source });
    // Get the Rebound holder from effect_state.target
    let rebound_holder = match battle.effect_state.borrow().target {
        Some(holder) => holder,
        None => return EventResult::Continue,
    };

    // Reflect the move: Rebound holder uses the move against the original source
    let move_data = match battle.dex.moves().get(move_id).cloned() {
        Some(m) => m,
        None => return EventResult::Continue,
    };
    crate::battle_actions::use_move(
        battle,
        &move_data,
        rebound_holder,  // Rebound holder becomes the user
        Some(source),    // Original source becomes the target
        None,            // No source effect
        None,            // Not a Z-move
        None,            // Not a Max move
    );

    // return null;
    EventResult::Null
}

